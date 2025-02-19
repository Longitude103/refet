use std::error::Error;
use chrono::{DateTime, NaiveDate, Utc};
use crate::conversions::*;
use crate::eta::EaInput;

pub struct Input {
    tmax: Value,
    tmin: Value,
    ea: Option<EaInput>,
    rs_input: Option<Value>,
    ws: Option<Value>,
    wz: Option<Value>,
    z: Option<Value>,
    latitude: Option<Value>,
    date: DateTime<Utc>,
}

impl Input {
    pub fn new(
        tmax: f64,
        tmin: f64,
        units: &str,
        date: &str
    ) -> Result<Input, Box<dyn Error>> {
        if tmin > tmax {
            return Err("Invalid temperature input: tmax must be greater than or equal to tmin".into());
        }

        let naive_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|_| "Invalid date format, must be YYYY-MM-DD")?;
        let naive_datetime = naive_date.and_hms_opt(0,0,0).unwrap();
        let date : DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);
        
        Ok(Input {
            tmax: Value::new(tmax, units.to_string()),
            tmin: Value::new(tmin, units.to_string()),
            rs_input: None,
            ea: None,
            ws: None,
            wz: None,
            z: None,
            latitude: None,
            date,
        })
    }
    
    pub fn set_ea(&mut self, ea: EaInput) {
        self.ea = Some(ea);
    }
    
    // could return an error if the input is invalid
    pub fn set_rs_input(&mut self, rs: f64, units: &str) -> Result<(), Box<dyn Error>> {
        if rs > 0.0 && !units.is_empty() {
            self.rs_input = Some(Value::new(rs, units.to_string()));
            Ok(())
        } else {
            Err("Invalid radiation input".into())
        }
    }
    
    pub fn set_ws(&mut self, ws: f64, units: &str) -> Result<(), Box<dyn Error>> {
        if ws > 0.0 && !units.is_empty() {
            self.ws = Some(Value::new(ws, units.to_string()));
            Ok(())
        } else {
            Err("Invalid wind speed input".into())
        }
    }
    
    pub fn set_wz(&mut self, wz: f64, units: &str) -> Result<(), Box<dyn Error>> {
        if wz > 0.0 && !units.is_empty() {
            self.wz = Some(Value::new(wz, units.to_string()));
            Ok(())
        } else {
            Err("Invalid wind height input".into())
        }
    }
    
    pub fn set_z(&mut self, z: f64, units: &str) -> Result<(), Box<dyn Error>> {
        if z > 0.0 &&!units.is_empty() {
            self.z = Some(Value::new(z, units.to_string()));
            Ok(())
        } else {
            Err("Invalid elevation input".into())
        }
    }
    
    pub fn set_latitude(&mut self, latitude: f64, units: &str) -> Result<(), Box<dyn Error>> {
        if (-90.0..=90.0).contains(&latitude) && !units.is_empty() {
            self.latitude = Some(Value::new(latitude, units.to_string()));
            Ok(())
        } else {
            Err("Invalid latitude input".into())
        }
    }
    
    
    pub fn set_date(&mut self, date: String) -> Result<(), Box<dyn Error>> {
        // check to make sure format is yyyy-MM-dd
        // parse the date and update day_of_year accordingly
        // if parsing fails, return an error
        let naive_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|_| "Invalid date format, must be YYYY-MM-DD")?;
        let naive_datetime = naive_date.and_hms_opt(0,0,0).unwrap();
        self.date = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);
        Ok(())
    }

    pub fn get_tmax(&self) -> Result<f64, Box<dyn Error>> {
        self.tmax.convert_temp()
    }

    pub fn get_tmin(&self) -> Result<f64, Box<dyn Error>> {
        self.tmin.convert_temp()
    }

    pub fn get_ea(&self) -> Result<f64, Box<dyn Error>> {
        if let Some(ea) = &self.ea {
            ea.ea()
        } else {
            Err("Actual vapor pressure input is required".into())
        }
    }

    pub fn get_rs_input(&self) -> Result<f64, Box<dyn Error>> {
        if let Some(rs) = &self.rs_input {
            rs.convert_radiation()
        } else {
            Err("Radiation input is required".into())
        }
    }

    pub fn get_ws(&self) -> Result<f64, Box<dyn Error>> {
        if let Some(ws) = &self.ws {
            ws.convert_speed()
        } else {
            Err("Wind speed input is required".into())
        }
    }

    pub fn get_wz(&self) -> Result<f64, Box<dyn Error>> {
        if let Some(wz) = &self.wz {
            wz.convert_z()
        } else {
            Err("Wind z input is required".into())
        }
    }

    pub fn get_z(&self) -> Result<f64, Box<dyn Error>> {
        if let Some(z) = &self.z {
            z.convert_z()
        } else {
            Err("Wind z input is required".into())
        }
    }

    pub fn get_latitude(&self) -> Result<f64, Box<dyn Error>> {
        if let Some(latitude) = &self.latitude {
            latitude.convert_latitude()
        } else {
            Err("Latitude input is required".into())
        }
    }

    pub fn get_date(&self) -> Result<DateTime<Utc>, Box<dyn Error>> {
        Ok(self.date)
    }
}

pub struct Value {
    value: f64,
    units: String,
}

impl Value {
    pub fn new(value: f64, units: String) -> Value {
        Value { value, units }
    }

    pub fn get_units(&self) -> String {
        self.units.to_lowercase()
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn convert_temp(&self) -> Result<f64, Box<dyn Error>> {
        let iu = self.get_units();
        if iu.starts_with('f') {
            let v = f_to_c(self.value);
            Ok(v)
        } else {
            Ok(self.value)
        }
    }

    pub fn convert_radiation(&self) -> Result<f64, Box<dyn Error>> {
        let iu = self.get_units();
        if iu.starts_with('l') {
            // langleys to megajoules
            let v = lang_to_mj(self.value);
            Ok(v)
        } else if iu.starts_with('w') {
            // watts to megajoules
            let v = watts_to_mj(self.value);
            Ok(v)
        } else {
            Ok(self.value)
        }
    }

    // convert wind speed from miles per hour to meters per second
    pub fn convert_speed(&self) -> Result<f64, Box<dyn Error>> {
        if self.units.eq_ignore_ascii_case("mph") {
            let v = mph_to_mps(self.value);
            Ok(v)
        } else {
            Ok(self.value)
        }
    }

    pub fn convert_z(&self) -> Result<f64, Box<dyn Error>> {
        let iu = self.get_units();
        // convert height from feet to meters
        if iu.starts_with('f') {
            let v = feet_to_meters(self.value);
            Ok(v)
        } else {
            Ok(self.value)
        }
    }

    pub fn convert_latitude(&self) -> Result<f64, Box<dyn Error>> {
        // convert latitude from degrees to radians
        let v = degrees_to_radians(self.value);
        Ok(v)
    }
}
