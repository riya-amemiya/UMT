//! Date manipulation utilities.
//!
//! This module provides various date manipulation functions including:
//! - Leap year checking
//! - Current time with timezone offset
//! - Date creation
//! - Age calculation from birthday
//! - Date range generation
//! - Day of week calculation
//! - Day name localization
//! - Timezone offset string formatting
//! - Date formatting

pub mod birthday;
pub mod date_range;
pub mod day_of_week;
pub mod format;
pub mod get_day;
pub mod get_timezone_offset_string;
pub mod is_leap_year;
pub mod new_date;
pub mod now;

pub use birthday::umt_birthday;
pub use date_range::{umt_date_range, umt_date_range_with_step};
pub use day_of_week::{umt_day_of_week, umt_today_day_of_week};
pub use format::{umt_format, umt_format_iso};
pub use get_day::{DayLanguage, umt_get_day, umt_get_day_en, umt_get_day_ja};
pub use get_timezone_offset_string::{
    umt_get_local_timezone_offset_string, umt_get_timezone_offset_string,
    umt_get_timezone_offset_string_compact,
};
pub use is_leap_year::umt_is_leap_year;
pub use new_date::{umt_new_date_int, umt_new_date_string};
pub use now::{umt_now, umt_now_jst};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_module_exports() {
        // Test that all exports are accessible
        assert!(umt_is_leap_year(2020));

        let _ = umt_now(0);
        let _ = umt_now_jst();

        let _ = umt_new_date_int(2021, 1, 1, None, None, None, None);
        let _ = umt_new_date_string("2021-01-01", None, None, None, None, None);

        let _ = umt_birthday(2000, 1, 1, None);

        let start = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 3, 0, 0, 0).unwrap();
        let _ = umt_date_range(start, end);
        let _ = umt_date_range_with_step(start, end, 1);

        let _ = umt_day_of_week(Some(2025), Some(1), Some(1), None);
        let _ = umt_today_day_of_week(None);

        let _ = umt_get_day(0, DayLanguage::En);
        let _ = umt_get_day_ja(0);
        let _ = umt_get_day_en(0);

        let _ = umt_get_timezone_offset_string(540);
        let _ = umt_get_timezone_offset_string_compact(540);
        let _ = umt_get_local_timezone_offset_string();

        let date = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let _ = umt_format(&date, "YYYY-MM-DD", 0);
        let _ = umt_format_iso(&date, 0);
    }
}
