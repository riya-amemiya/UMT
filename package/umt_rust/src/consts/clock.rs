//! Time duration constants in milliseconds.
//!
//! This module provides constants for common time durations expressed in milliseconds.

/// Number of milliseconds in one second.
pub const ONE_SECOND_MS: u64 = 1000;

/// Number of milliseconds in one minute.
pub const ONE_MINUTE_MS: u64 = ONE_SECOND_MS * 60; // 60_000

/// Number of milliseconds in one hour.
pub const ONE_HOUR_MS: u64 = ONE_MINUTE_MS * 60; // 3_600_000

/// Number of milliseconds in one day.
pub const ONE_DAY_MS: u64 = ONE_HOUR_MS * 24; // 86_400_000

/// Number of milliseconds in one week.
pub const ONE_WEEK_MS: u64 = ONE_DAY_MS * 7; // 604_800_000

/// Number of milliseconds in one month (28 days).
pub const ONE_MONTH_MS_28: u64 = ONE_DAY_MS * 28; // 2_419_200_000

/// Number of milliseconds in one month (29 days).
pub const ONE_MONTH_MS_29: u64 = ONE_DAY_MS * 29; // 2_505_600_000

/// Number of milliseconds in one month (30 days).
pub const ONE_MONTH_MS: u64 = ONE_DAY_MS * 30; // 2_592_000_000

/// Number of milliseconds in one month (31 days).
pub const ONE_MONTH_MS_31: u64 = ONE_DAY_MS * 31; // 2_678_400_000

/// Number of milliseconds in one year (365 days).
pub const ONE_YEAR_MS: u64 = ONE_DAY_MS * 365; // 31_536_000_000

/// Number of milliseconds in one year (366 days, leap year).
pub const ONE_YEAR_MS_366: u64 = ONE_DAY_MS * 366; // 31_622_400_000
