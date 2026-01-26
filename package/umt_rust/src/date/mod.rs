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
