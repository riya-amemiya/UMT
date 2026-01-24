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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_time_unit_values() {
        assert_eq!(ONE_SECOND_MS, 1000);
        assert_eq!(ONE_MINUTE_MS, 60_000);
        assert_eq!(ONE_HOUR_MS, 3_600_000);
        assert_eq!(ONE_DAY_MS, 86_400_000);
        assert_eq!(ONE_WEEK_MS, 604_800_000);
    }

    #[test]
    fn test_time_unit_relationships() {
        assert_eq!(ONE_MINUTE_MS, ONE_SECOND_MS * 60);
        assert_eq!(ONE_HOUR_MS, ONE_MINUTE_MS * 60);
        assert_eq!(ONE_DAY_MS, ONE_HOUR_MS * 24);
        assert_eq!(ONE_WEEK_MS, ONE_DAY_MS * 7);
    }

    #[test]
    fn test_month_variations() {
        assert_eq!(ONE_MONTH_MS_28, ONE_DAY_MS * 28);
        assert_eq!(ONE_MONTH_MS_29, ONE_DAY_MS * 29);
        assert_eq!(ONE_MONTH_MS, ONE_DAY_MS * 30);
        assert_eq!(ONE_MONTH_MS_31, ONE_DAY_MS * 31);
    }

    #[test]
    fn test_month_order() {
        assert!(ONE_MONTH_MS_28 < ONE_MONTH_MS_29);
        assert!(ONE_MONTH_MS_29 < ONE_MONTH_MS);
        assert!(ONE_MONTH_MS < ONE_MONTH_MS_31);
    }

    #[test]
    fn test_year_variations() {
        assert_eq!(ONE_YEAR_MS, ONE_DAY_MS * 365);
        assert_eq!(ONE_YEAR_MS_366, ONE_DAY_MS * 366);
    }

    #[test]
    fn test_year_relationship() {
        assert!(ONE_YEAR_MS < ONE_YEAR_MS_366);
        assert_eq!(ONE_YEAR_MS_366 - ONE_YEAR_MS, ONE_DAY_MS);
    }

    #[test]
    fn test_exact_month_values() {
        assert_eq!(ONE_MONTH_MS_28, 2_419_200_000);
        assert_eq!(ONE_MONTH_MS_29, 2_505_600_000);
        assert_eq!(ONE_MONTH_MS, 2_592_000_000);
        assert_eq!(ONE_MONTH_MS_31, 2_678_400_000);
    }

    #[test]
    fn test_exact_year_values() {
        assert_eq!(ONE_YEAR_MS, 31_536_000_000);
        assert_eq!(ONE_YEAR_MS_366, 31_622_400_000);
    }
}
