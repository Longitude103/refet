use climate::output::Output;
use climate::units::Units;
use std::error::Error;
use std::f64::consts::E;

pub enum Method {
    Direct,
    DewPoint,
    MaxMinRelativeHumidity,
    DailyMaxRelativeHumidity,
    DailyMinRelativeHumidity,
    DailyMinAirTemperature,
}

// EA (mean actual vapor pressure) has several calculation methods in ASCE Standarized, we support many but not all
// Methods supported:
// Direct - Ea directly measured by station that is in kilopascals
// DewPoint - measured or computed dew point in Celsius
// MaxMinRelativeHumidity - max, min relative humidity (add RHmax, RHmin this struct, add temp in struct as well)
// DailyMaxRelativeHumidity - daily maximum relative humidity (put in Value, add Tmin)
// DailyMinRelativeHumidity - daily minimum relative humidity (put in Value, add Tmax)
// DailyMinAirTemperature - daily minimum air temperature (put in Value, add Tmin)
pub struct EaInput {
    input: Option<f64>, // Ea in kPa or Dewpoint in Celsius otherwise None
    method: Method,     // method to calculate Ea from Method enum
    rhmax: Option<f64>, // daily maximum relative humidity in %
    rhmin: Option<f64>, // daily minimum relative humidity in %
    tmax: Option<f64>,  // daily maximum air temperature in Celsius
    tmin: Option<f64>,  // daily minimum air temperature in Celsius
}

impl EaInput {
    pub fn new_empty(method: Method) -> EaInput {
        EaInput {
            input: None,
            method,
            rhmax: None,
            rhmin: None,
            tmax: None,
            tmin: None,
        }
    }

    pub fn new_from_output(output: &Output) -> EaInput {
        // first option is Use Ea set from output
        if output.get_ea().is_some() {
            EaInput::new_direct(output.get_ea().unwrap(), "kPa")
        } else if output.get_dewpoint().is_some() {
            EaInput::new_dewpoint(output.get_dewpoint().unwrap(), "C")
        } else if output.get_rhmin().is_some() && output.get_rhmax().is_some() {
            EaInput::new_rhmax_min(
                output.get_rhmax().unwrap(),
                output.get_rhmin().unwrap(),
                "C",
                output.get_tmax(),
                output.get_tmin(),
                "C",
            )
        } else if output.get_rhmax().is_some() {
            EaInput::new_rhmax(output.get_rhmax().unwrap(), "C", output.get_tmax(), "C")
        } else if output.get_rhmin().is_some() {
            EaInput::new_rhmin(output.get_rhmin().unwrap(), "C", output.get_tmin(), "C")
        } else {
            EaInput::new_tmin(output.get_tmin(), "C")
        }
    }

    pub fn new_direct(input: f64, units: &str) -> EaInput {
        let mut direct_value = 0.0;
        if let Ok(unit) = Units::from_abbreviation(units) {
            match unit {
                Units::KiloPascals => direct_value = input,
                Units::Pascals => {
                    direct_value = Units::Pascals
                        .convert(input, &Units::KiloPascals)
                        .expect("Units conversion failed")
                }
                _ => panic!("Invalid units for EA Direct: {}", units),
            }
        } else {
            panic!("Invalid units: {}", units)
        };

        EaInput {
            input: Some(direct_value),
            method: Method::Direct,
            rhmax: None,
            rhmin: None,
            tmax: None,
            tmin: None,
        }
    }

    pub fn new_dewpoint(tdew: f64, units: &str) -> EaInput {
        let mut direct_value = 0.0;
        if let Ok(unit) = Units::from_abbreviation(units) {
            match unit {
                Units::Celsius => direct_value = tdew,
                Units::Fahrenheit => {
                    direct_value = Units::Fahrenheit
                        .convert(tdew, &Units::Celsius)
                        .expect("Units conversion failed")
                }
                _ => panic!("Invalid units for dewpoint: {}", units),
            }
        } else {
            panic!("Invalid units: {}", units)
        };

        EaInput {
            input: Some(direct_value),
            method: Method::DewPoint,
            rhmax: None,
            rhmin: None,
            tmax: None,
            tmin: None,
        }
    }

    pub fn new_rhmax_min(
        rhmax: f64,
        rhmin: f64,
        rh_units: &str,
        tmax: f64,
        tmin: f64,
        temp_units: &str,
    ) -> EaInput {
        let mut ea_input = EaInput::new_empty(Method::MaxMinRelativeHumidity);
        Units::from_abbreviation(rh_units).expect("Invalid units for relative humidity");
        ea_input.rhmax = Some(rhmax);
        ea_input.rhmin = Some(rhmin);

        let t_unit = Units::from_abbreviation(temp_units).expect("Invalid units for temperature");
        match t_unit {
            Units::Celsius => {
                ea_input.tmax = Some(tmax);
                ea_input.tmin = Some(tmin);
            }
            Units::Fahrenheit => {
                ea_input.tmax = Some(
                    Units::Fahrenheit
                        .convert(tmax, &Units::Celsius)
                        .expect("Units conversion failed"),
                );
                ea_input.tmin = Some(
                    Units::Fahrenheit
                        .convert(tmin, &Units::Celsius)
                        .expect("Units conversion failed"),
                );
            }
            _ => panic!("Invalid units for temperature"),
        }

        ea_input
    }

    pub fn new_rhmax(rhmax: f64, rh_units: &str, tmax: f64, temp_units: &str) -> EaInput {
        let mut ea_input = EaInput::new_empty(Method::DailyMaxRelativeHumidity);
        Units::from_abbreviation(rh_units).expect("Invalid units for relative humidity");
        ea_input.rhmax = Some(rhmax);

        let t_unit = Units::from_abbreviation(temp_units).expect("Invalid units for temperature");
        match t_unit {
            Units::Celsius => {
                ea_input.tmax = Some(tmax);
            }
            Units::Fahrenheit => {
                ea_input.tmax = Some(
                    Units::Fahrenheit
                        .convert(tmax, &Units::Celsius)
                        .expect("Units conversion failed"),
                );
            }
            _ => panic!("Invalid units for temperature"),
        }

        ea_input
    }

    pub fn new_rhmin(rhmin: f64, rh_units: &str, tmin: f64, temp_units: &str) -> EaInput {
        let mut ea_input = EaInput::new_empty(Method::DailyMinRelativeHumidity);
        Units::from_abbreviation(rh_units).expect("Invalid units for relative humidity");
        ea_input.rhmin = Some(rhmin);

        let t_unit = Units::from_abbreviation(temp_units).expect("Invalid units for temperature");
        match t_unit {
            Units::Celsius => {
                ea_input.tmin = Some(tmin);
            }
            Units::Fahrenheit => {
                ea_input.tmax = Some(
                    Units::Fahrenheit
                        .convert(tmin, &Units::Celsius)
                        .expect("Units conversion failed"),
                );
            }
            _ => panic!("Invalid units for temperature"),
        }

        ea_input
    }

    pub fn new_tmin(tmin: f64, units: &str) -> EaInput {
        let mut tmin_value = 0.0;
        if let Ok(unit) = Units::from_abbreviation(units) {
            match unit {
                Units::Celsius => tmin_value = tmin,
                Units::Fahrenheit => {
                    tmin_value = Units::Fahrenheit
                        .convert(tmin, &Units::Celsius)
                        .expect("Units conversion failed")
                }
                _ => panic!("Invalid units for tmin: {}", units),
            }
        } else {
            panic!("Invalid units: {}", units)
        };

        EaInput {
            input: None,
            method: Method::DailyMinAirTemperature,
            rhmax: None,
            rhmin: None,
            tmax: None,
            tmin: Some(tmin_value), // Use the converted value here
        }
    }

    // ea is a method to return the ea that can be used in the various parts of the app
    pub fn ea(&self) -> Result<f64, Box<dyn Error>> {
        let ea = match self.method {
            Method::Direct => self.get_ea()?,
            Method::DewPoint => self.convert_from_tdew()?,
            Method::MaxMinRelativeHumidity => self.convert_min_max_rh()?,
            Method::DailyMaxRelativeHumidity => self.convert_rhmax()?,
            Method::DailyMinRelativeHumidity => self.convert_rhmin()?,
            Method::DailyMinAirTemperature => self.convert_from_tmin()?,
        };

        Ok(ea)
    }

    /// Calculates the saturation vapor pressure at a given temperature using the formula: e0 = 0.6108 * e^((17.27 * t) / (t + 237.3)) (Eq. 7)
    fn eo(t: f64) -> f64 {
        0.6108 * E.powf((17.27 * t) / (t + 237.3))
    }

    fn get_ea(&self) -> Result<f64, Box<dyn Error>> {
        let value = self.input.ok_or("must have an input value")?;
        Ok(value)
    }

    fn convert_from_tdew(&self) -> Result<f64, Box<dyn Error>> {
        let value = self.input.ok_or("must have an input value")?;
        let ea = Self::eo(value); // Eq. 8
        Ok(ea)
    }

    // creates a saturation vapor pressure using the minimum temperature found in Appendix E: Equation E1
    fn convert_from_tmin(&self) -> Result<f64, Box<dyn Error>> {
        let tmin_v = self.tmin.ok_or("tmin must be a valid input")?;
        let ea = Self::eo(tmin_v - 3.0); // Eq. 8
        Ok(ea)
    }

    fn convert_min_max_rh(&self) -> Result<f64, Box<dyn Error>> {
        let tmax_v = self.tmax.ok_or("tmax must be a valid input")?;
        let tmin_v = self.tmin.ok_or("tmin must be a valid input")?;
        let rhmax = self.rhmax.ok_or("RHmax must have valid value")?;
        let rhmin = self.rhmin.ok_or("RHmin must have valid value")?;

        let rhmax = if rhmax > 1.0 { rhmax / 100.0 } else { rhmax };
        let rhmin = if rhmin > 1.0 { rhmin / 100.0 } else { rhmin };

        let ea = ((Self::eo(tmin_v) * rhmax) + (Self::eo(tmax_v) * rhmin)) / 2.0; // Eq. 11
        Ok(ea)
    }

    fn convert_rhmin(&self) -> Result<f64, Box<dyn Error>> {
        let tmin_v = self.tmin.ok_or("tmin must be a valid input")?;
        let rhmin = self.rhmin.ok_or("RHmin must have valid value")?;
        let rhmin = if rhmin > 1.0 { rhmin / 100.0 } else { rhmin };

        let ea = Self::eo(tmin_v) * rhmin; // Eq. 12
        Ok(ea)
    }

    fn convert_rhmax(&self) -> Result<f64, Box<dyn Error>> {
        let tmax_v = self.tmax.ok_or("tmax must be a valid input")?;
        let rhmax = self.rhmax.ok_or("RHmax must have valid value")?;
        let rhmax = if rhmax > 1.0 { rhmax / 100.0 } else { rhmax };

        let ea = Self::eo(tmax_v) * rhmax; // Eq. 13
        Ok(ea)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ea_method_1_ea() {
        // let input = Value::new(1000.0, "pa".to_string());
        let ea_input = EaInput::new_direct(1000.0, "pa");
        // let ea_input = EaInput::new(Some(input), Direct, None, None, None, None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1.0);

        // let input = Value::new(1.2, "kpa".to_string());
        let ea_input = EaInput::new_direct(1.2, "kpa");
        // let ea_input = EaInput::new(Some(input), Direct, None, None, None, None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 1.2).abs() < 0.0001);

        // let input = Value::new(3.2, "kpa".to_string());
        let ea_input = EaInput::new_direct(3.2, "kpa");
        // let ea_input = EaInput::new(Some(input), Direct, None, None, None, None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3.2);

        // let input = Value::new(2853.0, "pa".to_string());
        let ea_input = EaInput::new_direct(2853.0, "pa");
        // let ea_input = EaInput::new(Some(input), Direct, None, None, None, None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.853).abs() < 0.0001);
    }

    #[test]
    fn test_ea_method_2_dew() {
        // let input = Value::new(10.0, "c".to_string());
        let ea_input = EaInput::new_dewpoint(10.0, "c");
        // let ea_input = EaInput::new(Some(input), DewPoint, None, None, None, None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 1.228).abs() < 0.0001);

        // let input = Value::new(65.0, "f".to_string());
        let ea_input = EaInput::new_dewpoint(65.0, "f");
        // let ea_input = EaInput::new(Some(input), DewPoint, None, None, None, None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.1076).abs() < 0.0001);
    }

    #[test]
    fn test_ea_method_5_min_max_rh() {
        // let t_max = Value::new(32.0, "c".to_string());
        // let t_min = Value::new(25.0, "C".to_string());

        let ea_input = EaInput::new_rhmax_min(75.0, 45.0, "%", 32.0, 25.0, "c");
        // let ea_input = EaInput::new(None, MaxMinRelativeHumidity, Some(75.0), Some(45.0), Some(t_max), Some(t_min));

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.2577).abs() < 0.0001);

        // let t_max = Value::new(29.0, "c".to_string());
        // let t_min = Value::new(20.0, "c".to_string());

        let ea_input = EaInput::new_rhmax_min(85.0, 65.0, "%", 29.0, 20.0, "c");
        // let ea_input = EaInput::new(None, MaxMinRelativeHumidity, Some(85.0), Some(65.0), Some(t_max), Some(t_min));

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.2956).abs() < 0.0001);
    }

    #[test]
    fn test_ea_method_6_rh_max() {
        // let t_min = Value::new(25.0, "c".to_string());
        let ea_input = EaInput::new_rhmax(75.0, "%", 25.0, "c");
        // let ea_input = EaInput::new(None, DailyMaxRelativeHumidity, Some(75.0), None, None, Some(t_min));

        let result = ea_input.ea();
        // assert!(result.is_ok());
        assert!((result.unwrap() - 2.3758).abs() < 0.0001);

        // let t_min = Value::new(20.0, "c".to_string());
        let ea_input = EaInput::new_rhmax(85.0, "%", 20.0, "c");
        // let ea_input = EaInput::new(None, DailyMaxRelativeHumidity, Some(85.0), None, None, Some(t_min));

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 1.9875).abs() < 0.0001);
    }

    #[test]
    fn test_ea_method_7_rh_min() {
        // let t_max = Value::new(32.0, "c".to_string());
        let ea_input = EaInput::new_rhmin(45.0, "percent", 32.0, "c");
        // let ea_input = EaInput::new(None, DailyMinRelativeHumidity, None, Some(45.0), Some(t_max), None);

        let result = ea_input.ea();
        if result.is_err() {
            panic!("Test failed: {:?}", result.unwrap_err());
        }
        assert!((result.unwrap() - 2.1396).abs() < 0.0001);

        // let t_max = Value::new(29.0, "c".to_string());
        let ea_input = EaInput::new_rhmin(65.0, "percent", 29.0, "c");
        // let ea_input = EaInput::new(None, DailyMinRelativeHumidity, None, Some(65.0), Some(t_max), None);

        let result = ea_input.ea();
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.6036).abs() < 0.0001);
    }
}
