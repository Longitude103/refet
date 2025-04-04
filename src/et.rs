use crate::conversions::day_of_year;
use crate::EaInput;
use climate::output::Output;
use std::f64::consts::{E, PI};

/// Calculates the short and tall referece et for a given set of conditions.
///
/// # Arguments
///
/// * `Input` - The Input values for temperature, relative humidity, and air pressure.
///
/// # Returns
///
/// * a tuple containing the short and tall reference evapotranspiration.
pub fn calculate_ref_et(
    input: &Output
) -> (f64, f64) {
    const LAMDA: f64 = 0.408;
    const G: f64 = 0.0;
    let eta = EaInput::new_from_output(input);  // Creates a EaInput from the Input values, chooses the proper method based on the input data.

    // atmospheric pressure
    let atmospheric_pressure = calc_atmospheric_pressure(input.get_z());
    // println!("Atmospheric pressure: {}", atmospheric_pressure);

    // psycometric constant
    let gamma = psy_constant(atmospheric_pressure);
    // println!("Psycometric constant: {}", gamma);

    // mean temperature
    let mean_temperature = mean_temp(input.get_tmax(), input.get_tmin());
    // println!("Mean temperature: {}", mean_temperature);

    // slope of vapor pressure curve
    let delta = es_slope(mean_temperature);
    // println!("Slope of vapor pressure curve: {}", delta);

    // saturation vapor pressure
    let saturation_vapor_pressure = es(input.get_tmax(), input.get_tmin());
    // println!("Saturation vapor pressure: {}", saturation_vapor_pressure);

    // extraterrestrial radiation
    let extraterrestrial_radiation = calc_ra(input.get_latitude(), day_of_year(&input.get_date()).unwrap());
    // println!("Latitude: {}", input.get_latitude());
    // println!("Day of Year: {}", day_of_year(&input.get_date()).unwrap());
    // println!("Extraterrestrial radiation: {}", extraterrestrial_radiation);

    // clear sky radiation
    let clear_sky_radiation = calc_rso(extraterrestrial_radiation, input.get_z());
    // println!("Clear sky radiation: {}", clear_sky_radiation);

    let rs: f64;
    if let Some(mut rs_value) = input.get_rs() {
        rs = rs_value;
    } else {
        let harg_rs = calculate_hargreaves_samani_rs(input.get_tmax(), input.get_tmin(), extraterrestrial_radiation);
        // limit rs to clear sky radiation
        if harg_rs > clear_sky_radiation {
            rs = clear_sky_radiation;
        } else {
            rs = harg_rs;
        }
    };

    // fraction of clear day
    let fraction_of_clear_day = calc_fcd(clear_sky_radiation, rs);
    // println!("Fraction of clear day: {}", fraction_of_clear_day);

    // long-wave radiation
    let long_wave_radiation = calc_rnl(fraction_of_clear_day, eta.ea().unwrap(), input.get_tmax(), input.get_tmin());
    // println!("Long-wave radiation: {}", long_wave_radiation);

    // short-wave radiation
    let short_wave_radiation = calc_rns(rs);
    // println!("Short-wave radiation: {}", short_wave_radiation);

    let net_radiation = calc_rn(short_wave_radiation, long_wave_radiation);
    // println!("Net radiation: {}", net_radiation);

    let adjusted_wind_speed = calc_ws(input.get_ws().unwrap(), input.get_wz());
    // println!("Adjusted wind speed: {}", adjusted_wind_speed);

    let et_short_numerator = LAMDA * delta * (net_radiation - G)
        + gamma
        * (900.0 / (mean_temperature + 273.0))
        * adjusted_wind_speed
        * (saturation_vapor_pressure - input.get_ea().unwrap());
    let et_short_denominator = delta + gamma * (1.0 + 0.34 * adjusted_wind_speed);
    // println!("ET short-term numerator: {}", et_short_numerator);
    // println!("ET short-term denominator: {}", et_short_denominator);

    let et_tall_numerator = LAMDA * delta * (net_radiation - G)
        + gamma
        * (1600.0 / (mean_temperature + 273.0))
        * adjusted_wind_speed
        * (saturation_vapor_pressure - input.get_ea().unwrap());
    let et_tall_denominator = delta + gamma * (1.0 + 0.38 * adjusted_wind_speed);
    // println!("ET tall-term numerator: {}", et_tall_numerator);
    // println!("ET tall-term denominator: {}", et_tall_denominator);

    (
        et_short_numerator / et_short_denominator,
        et_tall_numerator / et_tall_denominator,
    )
}

/// Calculates the atmospheric pressure at a given altitude (z) in meters.
///
/// # Arguments
///
/// * `z` - The altitude in meters.
///
/// # Returns
///
/// The atmospheric pressure in Pascals.
fn calc_atmospheric_pressure(z: f64) -> f64 {
    let mut calc_1 = (293.0 - 0.0065 * z) / 293.0;
    calc_1 = calc_1.powf(5.26);
    calc_1 * 101.3
}

/// Calculates the psychrometric constant based on atmospheric pressure.
///
/// # Arguments
///
/// * `atmospheric_pressure` - The atmospheric pressure in Pascals.
///
/// # Returns
///
/// The psychrometric constant in Pascals.
fn psy_constant(atmospheric_pressure: f64) -> f64 {
    atmospheric_pressure * 0.000665
}

/// Calculates the mean temperature from the given maximum and minimum temperatures.
///
/// # Arguments
///
/// * `max_temp` - The maximum temperature in degrees Celsius.
/// * `min_temp` - The minimum temperature in degrees Celsius.
///
/// # Returns
///
/// The mean temperature in degrees Celsius.
fn mean_temp(max_temp: f64, min_temp: f64) -> f64 {
    (max_temp + min_temp) / 2.0
}

/// Calculates the slope of the vapor pressure curve (Eq. 5)
///
/// # Arguments
///
/// * `tmean` - The mean temperature in degrees Celsius.
///
/// # Returns
///
/// The slope of the vapor pressure curve at the given mean temperature.
fn es_slope(tmean: f64) -> f64 {
    let e = (17.27 * tmean) / (tmean + 237.3);
    let num = 2503.0 * e.exp();
    let denom = (tmean + 237.3).powi(2);

    num / denom
}

/// Calculates the daily Saturation Vapor Pressure (Eq. 6)
///
/// # Arguments
///
/// * `max_temp` - The maximum temperature in degrees Celsius.
/// * `min_temp` - The minimum temperature in degrees Celsius.
///
/// # Returns
///
/// The daily Saturation Vapor Pressure at the given maximum and minimum temperatures.
///
/// # Panics
///
/// This function will panic if the provided temperatures are not valid.
fn es(max_temp: f64, min_temp: f64) -> f64 {
    (eo(max_temp) + eo(min_temp)) / 2.0
}

// eo is a function to calculate the saturation vapor pressure at a given temperature (Eq. 7)
///
/// This function calculates the saturation vapor pressure at a given temperature.
///
/// # Arguments
///
/// * `temp` - The temperature in degrees Celsius.
///
/// # Returns
///
/// The saturation vapor pressure at the given temperature.
///
/// # Panics
///
/// This function will panic if the provided temperature is not valid.
fn eo(temp: f64) -> f64 {
    0.6108 * E.powf((17.27 * temp) / (temp + 237.3))
}

/// Calculates the inverse relative distance factor of the Earth to the Sun. Found in equation 23.
///
/// # Arguments
///
/// * `doy` - Day of the year.
///
/// # Returns
///
/// * The inverse relative distance factor.
fn inverse_rel_dist_factor(doy: u32) -> f64 {
    1.0 + 0.033 * ((2.0 * PI / 365.0) * doy as f64).cos() // Eq. 23
}

/// Calculates the solar declination. Found in equation 24.
///
/// # Arguments
///
/// * `doy` - Day of the year.
///
/// # Returns
///
/// * The solar declination.
fn solar_declin(doy: u32) -> f64 {
    0.409 * ((2.0 * PI / 365.0) * doy as f64 - 1.39).sin() // Eq. 24
}

/// Calculates the sunset hour angle. Found in equation 27.
///
/// # Arguments
///
/// * `lat` - Latitude in radians.
/// * `delta` - Solar declination.
///
/// # Returns
///
/// * The sunset hour angle.
fn sunset_hour_angle(lat: f64, delta: f64) -> f64 {
    (-1.0 * lat.tan() * delta.tan()).acos() // Eq. 27
}

/// Calculates the Extraterrestrial Radiation for 24-Hour Periods. Found in equation 21.
///
/// # Arguments
///
/// * `phi` - Latitude in radians.
/// * `doy` - Day of the year.
///
/// # Returns
///
/// * The Extraterrestrial Radiation for 24-Hour Periods.
fn calc_ra(latitude: f64, doy: u32) -> f64 {
    println!("Latitude: {}, DOY: {}", latitude, doy);
    let dr = inverse_rel_dist_factor(doy);
    let delta = solar_declin(doy);
    let omega = sunset_hour_angle(latitude, delta);
    println!("Dr: {}, delta: {}, omega: {}", dr, delta, omega);

    24.0 / PI
        * 4.92
        * dr
        * (omega * latitude.sin() * delta.sin() + latitude.cos() * delta.cos() * omega.sin()) // Eq. 21
}

/// Calculates the clear-sky solar radiation. Found in equation 19.
///
/// # Arguments
/// * `ra` - Extraterrestrial radiation.
/// * `z` - Station elevation in meters.
///
/// # Returns
/// * Clear-sky solar radiation.
///
/// # Formula
/// Uses the formula: RSO = (0.75 + 2e-5 * z) * ra
fn calc_rso(ra: f64, z: f64) -> f64 {
    (0.75 + 2e-5 * z) * ra
}

/// Calculates the fraction of clear day (FCD).
///
/// This function calculates the fraction of clear day (FCD) based on the clear-sky solar radiation (RSO) and the total solar radiation (RS).
///
/// # Arguments
///
/// * `rso` - The clear-sky solar radiation.
/// * `rs` - The total solar radiation.
///
/// # Returns
///
/// The fraction of clear day (FCD).
pub fn calc_fcd(rso: f64, rs: f64) -> f64 {
    let mut relative_solar_radiation = rs / rso;

    relative_solar_radiation = relative_solar_radiation.clamp(0.3, 1.0);
    relative_solar_radiation * 1.35 - 0.35
}

/// calc_rnl is a function to compute net long-wave radiation  equation 17.
///
/// # Arguments
///
/// * `fcd` - Cloudiness factor
/// * `ea` - Actual vapor pressure
/// * `tmax` - Maximum temperature in Celsius
/// * `tmin` - Minimum temperature in Celsius
///
/// # Returns
///
/// * Net long-wave radiation
fn calc_rnl(fcd: f64, ea: f64, tmax: f64, tmin: f64) -> f64 {
    const SIGMA: f64 = 4.901e-9;

    SIGMA * fcd * (0.34 - 0.14 * ea.sqrt()) * ((tmax + 273.16).powi(4) + (tmin + 273.16).powi(4))
        / 2.0
}

/// Calculates the net solar or short-wave radiation. Found in equation 16.
///
/// # Arguments
///
/// * `rs` - Incoming solar radiation
///
/// # Returns
///
/// Net solar radiation after accounting for albedo.
fn calc_rns(rs: f64) -> f64 {
    const ALPHA: f64 = 0.23;
    (1.0 - ALPHA) * rs
}

/// Calculates the net radiation (Rn) based on the incoming shortwave radiation (Rns) and
/// the outgoing longwave radiation (Rnl). Found in equation 15.
///
/// # Arguments
///
/// * `rns` - Incoming shortwave radiation (float64)
/// * `rnl` - Outgoing longwave radiation (float64)
///
/// # Returns
///
/// * `f64` - Net radiation (Rn)
fn calc_rn(rns: f64, rnl: f64) -> f64 {
    rns - rnl
}

/// Calculates the wind speed adjusted for the standard 2m height.
///
/// # Arguments
///
/// * `ws` - Wind speed at `wz` height in meters.
/// * `wz` - Height in meters where the wind speed `ws` is measured.
///
/// # Returns
///
/// * Adjusted wind speed at 2m height.
fn calc_ws(ws: f64, wz: f64) -> f64 {
    if wz == 2.0 {
        return ws;
    }

    ws * (4.87 / (67.8 * wz - 5.42).ln()) // Eq. 33
}

fn calculate_hargreaves_samani_rs(tmax: f64, tmin: f64, ra: f64) -> f64 {
    const ADJ_COEFFICIENT: f64 = 0.16;
    ADJ_COEFFICIENT * ra * (tmax - tmin).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atmospheric_pressure_greeley() {
        // Given
        let altitude = 1462.4; // negative altitude

        // When
        let atmospheric_pressure = calc_atmospheric_pressure(altitude);

        // greeley level based on the ASCE Standardized manual
        assert!((atmospheric_pressure - 85.1666).abs() < 0.001);
    }

    #[test]
    fn test_psy_constant() {
        //Given
        let atmospheric_pressure = 85.1666;

        // When
        let psy_constant = psy_constant(atmospheric_pressure);

        // greeley level based on the ASCE Standardized manual
        assert!((psy_constant - 0.056635).abs() < 0.001);
    }

    #[test]
    fn test_mean_temperature() {
        // Given
        let tmax = 32.4;
        let tmin = 10.9;

        // When
        let mean_temperature = mean_temp(tmax, tmin);

        // greeley level based on the ASCE Standardized manual
        assert!((mean_temperature - 21.65).abs() < 0.001);
    }

    #[test]
    fn test_es_slope() {
        // Given
        let average_temperature = 21.7;

        // When
        let es_slope = es_slope(average_temperature);

        // greeley level based on the ASCE Standardized manual
        assert!((es_slope - 0.1585).abs() < 0.001);
    }

    #[test]
    fn test_eo_max_temperature() {
        // Given
        let temperature = 32.4;

        // When
        let eo = eo(temperature);

        // greeley level based on the ASCE Standardized manual
        assert!((eo - 4.8633).abs() < 0.001);
    }
    #[test]
    fn test_eo_min_temperature() {
        // Given
        let temperature = 10.9;

        // When
        let eo = eo(temperature);

        // greeley level based on the ASCE Standardized manual
        assert!((eo - 1.30401).abs() < 0.001);
    }

    #[test]
    fn test_calculate_ws() {
        // Given
        let ws = 1.94;
        let wz = 3.0;

        // When
        let calculated_ws = calc_ws(ws, wz);

        // greeley level based on the ASCE Standardized manual
        assert!((calculated_ws - 1.786).abs() < 0.001);
    }

    #[test]
    fn test_inverse_rel_dist_factor() {
        // Given
        let z = 183;

        // When
        let inverse_rel_dist_factor = inverse_rel_dist_factor(z);

        // greeley level based on the ASCE Standardized manual
        assert!((inverse_rel_dist_factor - 0.967).abs() < 0.001);
    }

    #[test]
    fn test_solar_declin() {
        // Given
        let julian_day = 183;

        // When
        let solar_declin = solar_declin(julian_day);

        // greeley level based on the ASCE Standardized manual
        assert!((solar_declin - 0.4017).abs() < 0.001);
    }

    #[test]
    fn test_sunset_hour_angle() {
        // Given
        let solar_declin = 0.4017;
        let latitude = 40.41_f64.to_radians();

        // When
        let sunset_hour_angle = sunset_hour_angle(latitude, solar_declin);

        // greeley level based on the ASCE Standardized manual
        assert!((sunset_hour_angle - 1.941).abs() < 0.001);
    }

    #[test]
    fn test_calculate_ra() {
        // Given
        let latitude = 40.41_f64.to_radians();
        let julian_day = 183;

        // When
        let ra = calc_ra(latitude, julian_day);

        // greeley level based on the ASCE Standardized manual
        assert!((ra - 41.626).abs() < 0.001);
    }

    #[test]
    fn test_calculate_rso() {
        // Given
        let ra = 41.63;
        let z = 1462.4;

        // When
        let rso = calc_rso(ra, z);

        // greeley level based on the ASCE Standardized manual
        assert!((rso - 32.44).abs() < 0.001);
    }

    #[test]
    fn test_calculate_fcd() {
        // Given
        let rso = 32.44;
        let rs = 22.4;

        // When
        let fcd = calc_fcd(rso, rs);

        // greeley level based on the ASCE Standardized manual
        assert!((fcd - 0.5822).abs() < 0.001);
    }

    #[test]
    fn test_calculate_rnl() {
        // Given
        let fcd = 0.5822;
        let ea = 1.27;
        let tmax = 32.4;
        let tmin = 10.9;

        // When
        let rnl = calc_rnl(fcd, ea, tmax, tmin);

        // greeley level based on the ASCE Standardized manual
        assert!((rnl - 3.96).abs() < 0.001);
    }

    #[test]
    fn test_calculate_rns() {
        // Given
        let rs = 22.4;

        // When
        let rns = calc_rns(rs);

        // greeley level based on the ASCE Standardized manual
        assert!((rns - 17.247).abs() < 0.001);
    }

    #[test]
    fn test_calculate_rn() {
        // Given
        let rns = 17.247;
        let rnl = 3.96;

        // When
        let rn = calc_rn(rns, rnl);

        // greeley level based on the ASCE Standardized manual
        assert!((rn - 13.286).abs() < 0.001);
    }
}