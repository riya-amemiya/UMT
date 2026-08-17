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
//! - Duration arithmetic (add/subtract) and date differences
//! - Date boundaries (start-of/end-of unit)
//! - Relative-time formatting
//! - Calendar predicates (weekend, same-day, business day)
//! - Range checks, business-day arithmetic, quarter/week-of-year, unix time

pub mod add_business_days;
pub mod add_duration;
pub mod birthday;
pub mod date_inclusivity;
pub mod date_range;
pub mod day_of_week;
pub mod diff;
pub mod end_of;
pub mod format;
pub mod format_relative;
pub mod from_unix;
pub mod get_day;
pub mod get_quarter;
pub mod get_timezone_offset_string;
pub mod is_between;
pub mod is_business_day;
pub mod is_leap_year;
pub mod is_same_day;
pub mod is_weekend;
pub mod ms_by_unit;
pub mod new_date;
pub mod now;
pub mod start_of;
pub mod sub_business_days;
pub mod sub_duration;
pub mod to_unix;
pub mod unix_time_unit;
pub mod week_of_year;

pub use add_business_days::umt_add_business_days;
pub use add_duration::umt_add_duration;
pub use birthday::umt_birthday;
pub use date_inclusivity::DateInclusivity;
pub use date_range::{umt_date_range, umt_date_range_with_step};
pub use day_of_week::{umt_day_of_week, umt_today_day_of_week};
pub use diff::umt_diff;
pub use end_of::umt_end_of;
pub use format::{umt_format, umt_format_iso};
pub use format_relative::umt_format_relative;
pub use from_unix::umt_from_unix;
pub use get_day::{DayLanguage, umt_get_day, umt_get_day_en, umt_get_day_ja};
pub use get_quarter::umt_get_quarter;
pub use get_timezone_offset_string::{
    umt_get_local_timezone_offset_string, umt_get_timezone_offset_string,
    umt_get_timezone_offset_string_compact,
};
pub use is_between::umt_is_between;
pub use is_business_day::umt_is_business_day;
pub use is_leap_year::umt_is_leap_year;
pub use is_same_day::umt_is_same_day;
pub use is_weekend::umt_is_weekend;
pub use ms_by_unit::{DurationUnit, umt_ms_by_unit};
pub use new_date::{umt_new_date_int, umt_new_date_string};
pub use now::{umt_now, umt_now_jst};
pub use start_of::{DateBoundaryUnit, umt_start_of};
pub use sub_business_days::umt_sub_business_days;
pub use sub_duration::umt_sub_duration;
pub use to_unix::umt_to_unix;
pub use unix_time_unit::UnixTimeUnit;
pub use week_of_year::umt_week_of_year;
