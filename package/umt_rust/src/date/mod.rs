//! Date manipulation utilities.
//!
//! This module provides functions for working with dates, including:
//! - Leap year detection
//! - Day of week calculations
//! - Date formatting
//! - Date range generation
//! - Birthday/age calculations
//! - Timezone handling
//!
//! # Examples
//!
//! ```
//! use umt_rust::date::*;
//!
//! // Check if a year is a leap year
//! assert!(umt_is_leap_year(2024));
//! assert!(!umt_is_leap_year(2023));
//!
//! // Get day name in different languages
//! assert_eq!(umt_get_day_str(0, "en"), "Sun");
//! assert_eq!(umt_get_day_default(0), "日");
//! ```

pub mod birthday;
pub use birthday::*;

pub mod date_range;
pub use date_range::*;

pub mod day_of_week;
pub use day_of_week::*;

pub mod format;
pub use format::*;

pub mod get_day;
pub use get_day::*;

pub mod get_timezone_offset_string;
pub use get_timezone_offset_string::*;

pub mod is_leap_year;
pub use is_leap_year::*;

pub mod new_date;
pub use new_date::*;

pub mod now;
pub use now::*;
