mod conversions;
mod et;
mod eta;

pub use et::calculate_ref_et;
pub use eta::{EaInput, Method};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use climate::output::Output;

    #[test]
    fn test_calculate_ref_et() {
        // let naive_date = NaiveDate::from_ymd_opt(2000, 7, 1).unwrap();
        // let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
        let output = Output::new_with_values(
            32.4,
            10.9,
            None,
            None,
            None,
            Some(1.27),
            Some(22.4),
            Some(1.94),
            Some(3.0),
            1462.4,
            40.41_f64.to_radians(),
            Utc::now().date_naive(),
        );
        let (short_et, tall_et) = calculate_ref_et(&output);

        println!("Short-term ET: {}", short_et);
        println!("Tall-term ET: {}", tall_et);
    }
}
