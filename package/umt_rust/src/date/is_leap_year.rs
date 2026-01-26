//! Leap year checking utility.
//!
//! This module provides a function to determine if a given year is a leap year.

/// Determine if a given year is a leap year.
///
/// A year is a leap year if:
/// - It is divisible by 4 AND not divisible by 100, OR
/// - It is divisible by 400
///
/// # Arguments
///
/// * `year` - The year to check
///
/// # Returns
///
/// `true` if the year is a leap year, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_is_leap_year;
///
/// assert!(umt_is_leap_year(2000));  // divisible by 400
/// assert!(umt_is_leap_year(2020));  // divisible by 4 but not 100
/// assert!(!umt_is_leap_year(2100)); // divisible by 100 but not 400
/// assert!(!umt_is_leap_year(2023)); // not divisible by 4
/// ```
#[inline]
pub fn umt_is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leap_year_divisible_by_400() {
        assert!(umt_is_leap_year(2000));
        assert!(umt_is_leap_year(1600));
        assert!(umt_is_leap_year(2400));
    }

    #[test]
    fn test_leap_year_divisible_by_4_not_100() {
        assert!(umt_is_leap_year(2020));
        assert!(umt_is_leap_year(2024));
        assert!(umt_is_leap_year(2028));
        assert!(umt_is_leap_year(1996));
    }

    #[test]
    fn test_not_leap_year_divisible_by_100_not_400() {
        assert!(!umt_is_leap_year(2100));
        assert!(!umt_is_leap_year(1900));
        assert!(!umt_is_leap_year(2200));
        assert!(!umt_is_leap_year(2300));
    }

    #[test]
    fn test_not_leap_year_not_divisible_by_4() {
        assert!(!umt_is_leap_year(2023));
        assert!(!umt_is_leap_year(2021));
        assert!(!umt_is_leap_year(2019));
        assert!(!umt_is_leap_year(2001));
    }

    #[test]
    fn test_negative_years() {
        assert!(umt_is_leap_year(-4));
        assert!(!umt_is_leap_year(-1));
        assert!(umt_is_leap_year(-400));
    }
}
