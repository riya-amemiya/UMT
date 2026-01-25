//! Integration tests for Date utility functions
//!
//! Tests the interaction between date operations and formatting:
//! - Date range generation with formatting
//! - Birthday calculations with day of week
//! - Complex date workflows

use chrono::{Datelike, TimeZone, Utc};
use umt_rust::date::{
    umt_birthday, umt_date_range, umt_day_of_week, umt_format, umt_get_day_en, umt_is_leap_year,
    umt_new_date_int,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_date_range_and_format_each_date() {
        let start_date = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2025, 1, 5, 0, 0, 0).unwrap();
        let dates = umt_date_range(start_date, end_date);

        let formatted_dates: Vec<String> =
            dates.iter().map(|date| umt_format(date, "YYYY-MM-DD", 0)).collect();

        assert_eq!(
            formatted_dates,
            vec!["2025-01-01", "2025-01-02", "2025-01-03", "2025-01-04", "2025-01-05"]
        );
    }

    #[test]
    fn should_calculate_age_and_format_birthday_with_day_of_week() {
        let birth_year = 2000;
        let birth_month = 3;
        let birth_day = 15;

        let age = umt_birthday(birth_year as i32, birth_month, birth_day, None);
        let day_num = umt_day_of_week(
            Some(birth_year as i32),
            Some(birth_month),
            Some(birth_day),
            None,
        );
        let day_name = umt_get_day_en(day_num);

        let birth_date = umt_new_date_int(birth_year as i32, birth_month, birth_day, None, None, None, None);
        let formatted_birth = umt_format(&birth_date, "MM/DD/YYYY", 0);

        assert!(age > 20, "Age should be greater than 20, got {}", age);
        assert_eq!(day_name, "Wed");
        assert_eq!(formatted_birth, "03/15/2000");
    }

    #[test]
    fn should_generate_weekday_dates_within_a_month() {
        let year = 2025;
        let month = 1;
        let start_date = umt_new_date_int(year, month, 1, None, None, None, None);
        let end_date = umt_new_date_int(year, month, 31, None, None, None, None);

        let all_dates = umt_date_range(start_date, end_date);
        let weekday_dates: Vec<_> = all_dates
            .iter()
            .filter(|date| {
                let day = date.weekday().num_days_from_sunday();
                day != 0 && day != 6
            })
            .collect();

        let formatted_weekdays: Vec<_> = weekday_dates
            .iter()
            .map(|date| {
                let formatted = umt_format(date, "YYYY-MM-DD", 0);
                let day_name = umt_get_day_en(date.weekday().num_days_from_sunday() as u8);
                (formatted, day_name)
            })
            .collect();

        assert_eq!(
            formatted_weekdays.len(),
            23,
            "Expected 23 weekdays, got {}",
            formatted_weekdays.len()
        );

        let valid_days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
        for (_, day) in &formatted_weekdays {
            assert!(
                valid_days.contains(&day.as_str()),
                "Day {} is not a weekday",
                day
            );
        }
    }

    #[test]
    fn should_handle_leap_year_dates_correctly() {
        let leap_years = [2020, 2024, 2028];
        let non_leap_years = [2021, 2022, 2023];

        for year in leap_years {
            let feb1 = umt_new_date_int(year, 2, 1, None, None, None, None);
            let feb29 = umt_new_date_int(year, 2, 29, None, None, None, None);
            let dates = umt_date_range(feb1, feb29);

            let is_leap = umt_is_leap_year(year as u32);
            let feb_days = dates.len();
            let last_day = umt_format(&dates[dates.len() - 1], "YYYY-MM-DD", 0);

            assert!(is_leap, "Year {} should be a leap year", year);
            assert_eq!(feb_days, 29, "February {} should have 29 days", year);
            assert_eq!(last_day, format!("{}-02-29", year));
        }

        for year in non_leap_years {
            assert!(
                !umt_is_leap_year(year as u32),
                "Year {} should not be a leap year",
                year
            );
        }
    }

    #[test]
    fn should_create_calendar_view_with_formatted_dates_and_day_names() {
        let year = 2025;
        let month = 4;
        let first_day = umt_new_date_int(year, month, 1, None, None, None, None);
        let last_day = umt_new_date_int(year, month, 30, None, None, None, None);

        let dates = umt_date_range(first_day, last_day);
        let calendar_data: Vec<_> = dates
            .iter()
            .map(|date| {
                let day_of_month = date.day();
                (
                    umt_format(date, "DD", 0),
                    umt_get_day_en(date.weekday().num_days_from_sunday() as u8),
                    umt_format(date, "YYYY-MM-DD", 0),
                    ((day_of_month - 1) / 7 + 1) as u32,
                )
            })
            .collect();

        assert_eq!(calendar_data[0].0, "01");
        assert_eq!(calendar_data[0].1, "Tue");
        assert_eq!(calendar_data[0].2, "2025-04-01");
        assert_eq!(calendar_data[0].3, 1);

        let last_idx = calendar_data.len() - 1;
        assert_eq!(calendar_data[last_idx].0, "30");
        assert_eq!(calendar_data[last_idx].1, "Wed");
        assert_eq!(calendar_data[last_idx].2, "2025-04-30");
        assert_eq!(calendar_data[last_idx].3, 5);
    }

    #[test]
    fn should_calculate_business_days_between_dates() {
        let start_date = umt_new_date_int(2025, 1, 1, None, None, None, None);
        let end_date = umt_new_date_int(2025, 1, 31, None, None, None, None);

        let all_dates = umt_date_range(start_date, end_date);
        let business_days: Vec<_> = all_dates
            .iter()
            .filter(|date| {
                let day_num = date.weekday().num_days_from_sunday();
                day_num != 0 && day_num != 6
            })
            .collect();

        let formatted_business_days: Vec<String> = business_days
            .iter()
            .map(|date| umt_format(date, "MM/DD", 0))
            .collect();

        assert_eq!(business_days.len(), 23);
        assert_eq!(formatted_business_days[0], "01/01");
        assert_eq!(
            formatted_business_days[formatted_business_days.len() - 1],
            "01/31"
        );
    }

    #[test]
    fn should_handle_international_date_formatting() {
        let test_date = umt_new_date_int(2025, 12, 25, None, None, None, None);

        let us_format = umt_format(&test_date, "MM/DD/YYYY", 0);
        let eu_format = umt_format(&test_date, "DD/MM/YYYY", 0);
        let iso_format = umt_format(&test_date, "YYYY-MM-DD", 0);
        let simple_format = umt_format(&test_date, "MM/DD", 0);

        let day_of_week_num =
            umt_day_of_week(Some(2025), Some(12), Some(25), None);
        let day_name = umt_get_day_en(day_of_week_num);
        let day_of_week_format = format!("{}, {}", day_name, simple_format);

        assert_eq!(us_format, "12/25/2025");
        assert_eq!(eu_format, "25/12/2025");
        assert_eq!(iso_format, "2025-12-25");
        assert_eq!(simple_format, "12/25");
        assert_eq!(day_of_week_format, "Thu, 12/25");
    }
}
