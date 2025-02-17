use std::error::Error;
use crate::conversions::*;
use crate::eta::EaInput;

pub struct Input {
    tmax: Value,
    tmin: Value,
    ea: Option<EaInput>,
    rs_input: f64,
    ws: Value,
    wz: Value,
    z: Value,
    latitude: Value,
    day_of_year: Value,
}

impl Input {
    pub fn new(
        tmax: Value,
        tmin: Value,
        ea: None,
        rs_input: f64,
        ws: Value,
        wz: Value,
        z: Value,
        latitude: Value,
        day_of_year: Value,
    ) -> Input {
        Input {
            tmax,
            tmin,
            ea,
            rs_input,
            ws,
            wz,
            z,
            latitude,
            day_of_year,
        }
    }

    pub fn get_tmax(&self) -> Result<f64, Box<dyn Error>> {
        self.convert_temp()
    }

    pub fn get_tmin(&self) -> Result<f64, Box<dyn Error>> {
        self.convert_temp()
    }

    pub fn get_ea(&self) -> EaInput {
        self.ea
    }

    pub fn get_rs_input(&self) -> Result<f64, Box<dyn Error>> {
        self.convert_radiation()
    }

    pub fn get_ws(&self) -> Result<f64, Box<dyn Error>> {
        self.convert_speed()
    }

    pub fn get_wz(&self) -> Result<f64, Box<dyn Error>> {
        self.convert_z()
    }

    pub fn get_z(&self) -> Result<f64, Box<dyn Error>> {
        self.convert_z()
    }

    pub fn get_latitude(&self) -> Value {
        self.latitude
    }

    pub fn get_day_of_year(&self) -> Value {
        self.day_of_year
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

    pub fn convert_date_to_day_of_year(&self) -> Result<u32, Box<dyn Error>> {
        let v = day_of_year(&self.value.to_string())?;
        Ok(v)
    }
}
