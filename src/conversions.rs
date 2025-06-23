use chrono::{DateTime, Datelike, Utc};

// pub fn c_to_f(value: f64) -> f64 {
//     // conversion of Celsius to Fahrenheit
//     value * 9.0 / 5.0 + 32.0
// }

// pub fn f_to_c(value: f64) -> f64 {
//     // conversion of Fahrenheit to celsius
//     (value - 32.0) * 5.0 / 9.0
// }

// pub fn pa_to_kpa(value: f64) -> f64 {
//     // conversion of pascals to kilopascals
//     value / 1000.0
// }

// pub fn lang_to_mj(value: f64) -> f64 {
//     // conversion of megajoules to watts
//     value * 0.04184
// }

// pub fn mj_to_lang(value: f64) -> f64 {
//     // conversion of megajoules to watts
//     value / 0.04184
// }

// pub fn watts_to_mj(value: f64) -> f64 {
//     // conversion of watts to megajoules
//     value * 0.0864
// }

// pub fn mph_to_mps(value: f64) -> f64 {
//     // conversion of miles per hour to meters per second
//     value * 0.44704
// }

// pub fn mps_to_mph(value: f64) -> f64 {
//     // conversion of meters per second to miles per hour
//     value / 0.44704
// }

// pub fn feet_to_meters(value: f64) -> f64 {
//     // conversion of feet to meters
//     value * 0.3048
// }

// pub fn degrees_to_radians(degrees: f64) -> f64 {
//     // conversion of degrees to radians
//     degrees * PI / 180.0
// }

// fn radians_to_degrees(radians: f64) -> f64 {
//     // conversion of radians to degrees
//     radians * 180.0 / PI
// }

/// Converts a given date (in the format yyyy-mm-dd) to the day of the year.
///
/// # Arguments
/// * `date_str` - A string slice that holds the date in the format "yyyy-mm-dd".
///
/// # Returns
/// * A Result that is either:
///   - Ok(u32): the day of the year as a u32 if the input date is valid.
///   - Err(String): an error string indicating what went wrong (e.g., invalid date format).
///
pub fn day_of_year(date: &DateTime<Utc>) -> Result<u32, String> {
    // Get the day of the year
    Ok(date.ordinal())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDate, Utc};

    #[test]
    fn test_day_of_year() {
        let naive_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
        let day_of_year =
            day_of_year(&DateTime::from_naive_utc_and_offset(naive_datetime, Utc)).unwrap();
        assert_eq!(day_of_year, 1);
    }

    #[test]
    fn test_day_of_year_leap_year() {
        let naive_date = NaiveDate::from_ymd_opt(2020, 2, 29).unwrap();
        let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
        let day_of_year =
            day_of_year(&DateTime::from_naive_utc_and_offset(naive_datetime, Utc)).unwrap();
        assert_eq!(day_of_year, 60);
    }

    // #[test]
    // fn test_c_to_f() {
    //     assert_eq!(c_to_f(0.0), 32.0);
    //     assert_eq!(c_to_f(100.0), 212.0);
    //     assert_eq!(c_to_f(-40.0), -40.0);
    // }

    // #[test]
    // fn test_f_to_c() {
    //     assert_eq!(f_to_c(32.0), 0.0);
    //     assert_eq!(f_to_c(212.0), 100.0);
    //     assert_eq!(f_to_c(-40.0), -40.0);
    // }

    // #[test]
    // fn test_pa_to_kpa() {
    //     assert_eq!(pa_to_kpa(1000.0), 1.0);
    //     assert_eq!(pa_to_kpa(250000.0), 250.0);
    //     assert_eq!(pa_to_kpa(0.0), 0.0);
    // }

    // #[test]
    // fn test_lang_to_mj() {
    //     assert_eq!(lang_to_mj(1000.0), 41.84);
    //     assert_eq!(lang_to_mj(250000.0), 10460.0);
    //     assert_eq!(lang_to_mj(0.0), 0.0);
    // }

    // #[test]
    // fn test_mj_to_lang() {
    //     let value = ((mj_to_lang(1000.0) * 100.0).round()) / 100.0;
    //     assert_eq!(value, 23900.57);

    //     let value = ((mj_to_lang(25000.0) * 100.0).round()) / 100.0;
    //     assert_eq!(value, 597514.34);
    //     assert_eq!(mj_to_lang(0.0), 0.0);
    // }

    // #[test]
    // fn test_mph_to_mps() {
    //     assert_eq!(mph_to_mps(0.0), 0.0);
    //     assert_eq!(mph_to_mps(25.0), 11.176);
    //     assert_eq!(mph_to_mps(75.0), 33.528);
    // }

    // #[test]
    // fn test_mps_to_mph() {
    //     assert_eq!(mps_to_mph(0.0), 0.0);
    //     assert_eq!(mps_to_mph(11.176), 25.0);
    //     assert_eq!(mps_to_mph(33.528), 75.0);
    // }

    // #[test]
    // fn test_feet_to_meters() {
    //     assert_eq!(feet_to_meters(0.0), 0.0);

    //     let value = ((feet_to_meters(3.0) * 10000.0).round()) / 10000.0;
    //     assert_eq!(value, 0.9144);

    //     let value = ((feet_to_meters(12.0) * 10000.0).round()) / 10000.0;
    //     assert_eq!(value, 3.6576);
    // }

    // #[test]
    // fn test_degrees_to_radians() {
    //     assert_eq!(degrees_to_radians(0.0), 0.0);
    //     assert_eq!(degrees_to_radians(90.0), PI / 2.0);
    //     assert_eq!(degrees_to_radians(180.0), PI);
    //     assert_eq!(degrees_to_radians(270.0), 3.0 * PI / 2.0);
    //     assert_eq!(degrees_to_radians(360.0), 2.0 * PI);
    // }

    // #[test]
    // fn test_radians_to_degrees() {
    //     assert_eq!(radians_to_degrees(0.0), 0.0);
    //     assert_eq!(radians_to_degrees(PI / 2.0), 90.0);
    //     assert_eq!(radians_to_degrees(PI), 180.0);
    //     assert_eq!(radians_to_degrees(3.0 * PI / 2.0), 270.0);
    //     assert_eq!(radians_to_degrees(2.0 * PI), 360.0);
    // }
}
