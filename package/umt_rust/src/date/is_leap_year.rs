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
