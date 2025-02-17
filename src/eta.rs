#![allow(dead_code)]

use super::conversions::pa_to_kpa;
use super::input::Input;
use std::error::Error;
use std::f64::consts::E;

// EA (mean actual vapor pressure) has several calculation methods in ASCE Standarized, we support many but not all
// Methods supported:
// 1 - Ea directly measured by station
// 2 - measured or computed dew point
// 5 - max, min relative humidity (add RHmax, RHmin this struct, add temp in struct as well)
// 6 - daily maximum relative humidty (put in Value, add Tmin)
// 7 - daily minimum relative humidty (put in Value, add Tmax)
// 8 - daily minimum air temperature (put in Value, add Tmin)
pub struct EaInput {
    input: Option<Input>,
    method: i32,
    rhmax: Option<f64>,
    rhmin: Option<f64>,
    tmax: Option<Input>,
    tmin: Option<Input>,
}

impl EaInput {
    pub fn new(
        input: Option<Input>,
        method: i32,
        rhmax: Option<f64>,
        rhmin: Option<f64>,
        tmax: Option<Input>,
        tmin: Option<Input>,
    ) -> EaInput {
        EaInput {
            input,
            method,
            rhmax,
            rhmin,
            tmax,
            tmin,
        }
    }

    // ea is a method to return the ea that can be used in the various parts of the app
    pub fn ea(&self) -> Result<f64, Box<dyn Error>> {
        let ea = match self.method {
            1 => self.convert_ea()?,
            2 => self.convert_from_tdew()?,
            5 => self.convert_min_max_rh()?,
            6 => self.convert_rhmax()?,
            7 => self.convert_rhmin()?,
            8 => self.convert_from_tmin()?,
            _ => return Err("Invalid method".into()),
        };

        Ok(ea)
    }

    /// Calculates the saturation vapor pressure at a given temperature using the formula: e0 = 0.6108 * e^((17.27 * t) / (t + 237.3)) (Eq. 7)
    fn eo(t: f64) -> f64 {
        0.6108 * E.powf((17.27 * t) / (t + 237.3))
    }

    fn convert_ea(&self) -> Result<f64, Box<dyn Error>> {
        let v = self
            .input
            .as_ref()
            .ok_or("must have an input value and units")?;

        let iu = v.get_units();
        if iu.starts_with('p') {
            let v = pa_to_kpa(v.get_value());
            return Ok(v);
        } else {
            return Ok(v.get_value());
        }
    }

    fn convert_from_tdew(&self) -> Result<f64, Box<dyn Error>> {
        let v = self
            .input
            .as_ref()
            .ok_or("must have a input value and untis")?
            .convert_temp()?;
        let ea = Self::eo(v); // Eq. 8
        Ok(ea)
    }
    
    // creates a saturation vapor pressure using the minimum temperature found in Appendix E: Equation E1
    fn convert_from_tmin(&self) -> Result<f64, Box<dyn Error>> {
        let tmin_v = self
           .tmin
           .as_ref()
           .ok_or("tmin must be a valid input")?
           .convert_temp()?;
        let ea = Self::eo(tmin_v - 3.0); // Eq. 8
        Ok(ea)
    }

    fn convert_min_max_rh(&self) -> Result<f64, Box<dyn Error>> {
        let tmax_v = self
            .tmax
            .as_ref()
            .ok_or("tmax must be a valid input")?
            .convert_temp()?;
        let tmin_v = self
            .tmin
            .as_ref()
            .ok_or("tmin must be a valid input")?
            .convert_temp()?;

        let rhmax = self.rhmax.ok_or("RHmax must have valid value")?;
        let rhmin = self.rhmin.ok_or("RHmin must have valid value")?;

        let rhmax = if rhmax > 1.0 { rhmax / 100.0 } else { rhmax };
        let rhmin = if rhmin > 1.0 { rhmin / 100.0 } else { rhmin };

        let ea = ((Self::eo(tmin_v) * rhmax) + (Self::eo(tmax_v) * rhmin)) / 2.0; // Eq. 11
        Ok(ea)
    }

    fn convert_rhmax(&self) -> Result<f64, Box<dyn Error>> {
        let tmin_v = self
            .tmin
            .as_ref()
            .ok_or("tmin must be a valid input")?
            .convert_temp()?;

        let rhmax = self.rhmax.ok_or("RHmax must have valid value")?;
        let rhmax = if rhmax > 1.0 { rhmax / 100.0 } else { rhmax };

        let ea = Self::eo(tmin_v) * rhmax; // Eq. 12
        Ok(ea)
    }

    fn convert_rhmin(&self) -> Result<f64, Box<dyn Error>> {
        let tmax_v = self
            .tmax
            .as_ref()
            .ok_or("tmax must be a valid input")?
            .convert_temp()?;

        let rhmin = self.rhmin.ok_or("RHmin must have valid value")?;
        let rhmin = if rhmin > 1.0 { rhmin / 100.0 } else { rhmin };

        let ea = Self::eo(tmax_v) * rhmin; // Eq. 13
        Ok(ea)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ea_method_1_ea() {
        let input = Input::new(1000.0, "pa".to_string());
        let ea_input = EaInput::new(Some(input), 1, None, None, None, None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1.0);
        
        let input = Input::new(1.2, "kpa".to_string());
        let ea_input = EaInput::new(Some(input), 1, None, None, None, None);
        
        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 1.2).abs() < 0.0001);

        let input = Input::new(3.2, "kpa".to_string());
        let ea_input = EaInput::new(Some(input), 1, None, None, None, None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3.2);

        let input = Input::new(2853.0, "pa".to_string());
        let ea_input = EaInput::new(Some(input), 1, None, None, None, None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.853).abs() < 0.0001);
    }

    #[test]
    fn test_ea_method_2_dew() {
        let input = Input::new(10.0, "c".to_string());
        let ea_input = EaInput::new(Some(input), 2, None, None, None, None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 1.228).abs() < 0.0001);

        let input = Input::new(65.0, "f".to_string());
        let ea_input = EaInput::new(Some(input), 2, None, None, None, None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.1076).abs() < 0.0001);
    }

    #[test]
    fn test_ea_method_5_min_max_rh() {
        let t_max = Input::new(32.0, "c".to_string());
        let t_min = Input::new(25.0, "C".to_string());

        let ea_input = EaInput::new(None, 5, Some(75.0), Some(45.0), Some(t_max), Some(t_min));

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.2577).abs() < 0.0001);

        let t_max = Input::new(29.0, "c".to_string());
        let t_min = Input::new(20.0, "c".to_string());

        let ea_input = EaInput::new(None, 5, Some(85.0), Some(65.0), Some(t_max), Some(t_min));

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.2956).abs() < 0.0001);
    }

    #[test]
    fn test_ea_method_6_rh_max() {
        let t_min = Input::new(25.0, "c".to_string());
        let ea_input = EaInput::new(None, 6, Some(75.0), None, None, Some(t_min));

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.3758).abs() < 0.0001);

        let t_min = Input::new(20.0, "c".to_string());
        let ea_input = EaInput::new(None, 6, Some(85.0), None, None, Some(t_min));

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 1.9875).abs() < 0.0001);
    }

    #[test]
    fn test_ea_method_7_rh_min() {
        let t_max = Input::new(32.0, "c".to_string());
        let ea_input = EaInput::new(None, 7, None, Some(45.0), Some(t_max), None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.1396).abs() < 0.0001);

        let t_max = Input::new(29.0, "c".to_string());
        let ea_input = EaInput::new(None, 7, None, Some(65.0), Some(t_max), None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.6036).abs() < 0.0001);
    }

    #[test]
    fn test_ea_invalid_method() {
        let ea_input = EaInput::new(None, 0, None, None, None, None);

        let result = ea_input.ea();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid method");
    }
}
