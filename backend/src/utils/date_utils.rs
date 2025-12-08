use chrono::{DateTime, Utc, NaiveDate, Duration};
use rust_decimal::Decimal;
use anyhow::Result;

pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    Ok(date)
}

pub fn format_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn calculate_depreciation(
    initial_value: Decimal,
    useful_life_years: i32,
    years_elapsed: i32,
    depreciation_method: &str,
) -> Result<Decimal> {
    match depreciation_method {
        "straight_line" => {
            if years_elapsed >= useful_life_years {
                return Ok(Decimal::ZERO);
            }
            let depreciation_per_year = initial_value / Decimal::from(useful_life_years);
            let total_depreciation = depreciation_per_year * Decimal::from(years_elapsed);
            let current_value = initial_value - total_depreciation;
            Ok(if current_value < Decimal::ZERO { Decimal::ZERO } else { current_value })
        },
        "double_declining" => {
            let mut current_value = initial_value;
            let rate = Decimal::from(2) / Decimal::from(useful_life_years);

            for _ in 0..years_elapsed.min(useful_life_years - 1) {
                let depreciation = current_value * rate;
                current_value = current_value - depreciation;
            }

            Ok(if current_value < Decimal::ZERO { Decimal::ZERO } else { current_value })
        },
        _ => Err(anyhow::anyhow!("Unsupported depreciation method: {}", depreciation_method))
    }
}

pub fn calculate_age_in_days(start_date: NaiveDate) -> i64 {
    let today = Utc::now().date_naive();
    (today - start_date).num_days()
}