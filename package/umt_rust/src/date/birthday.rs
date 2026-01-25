//! Birthday age calculation utility.
//!
//! This module provides a function to calculate age based on birthdate.

use chrono::{Datelike, NaiveDate};

use super::now::umt_now;

/// Calculate age based on birthdate.
///
/// # Arguments
///
/// * `year` - Birth year
/// * `month` - Birth month (1-12)
/// * `day` - Birth day (1-31)
/// * `time_difference` - Time difference from UTC in hours (default: 9 for JST)
///
/// # Returns
///
/// Age in years. Returns 0 if the birthdate is in the future.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_birthday;
///
/// // Calculate age of someone born on Jan 1, 2000
/// let age = umt_birthday(2000, 1, 1, None);
/// // age will depend on current date
/// ```
pub fn umt_birthday(year: i32, month: u32, day: u32, time_difference: Option<i32>) -> u32 {
    let tz_offset = time_difference.unwrap_or(9);
    let now = umt_now(tz_offset);

    let birth_date = match NaiveDate::from_ymd_opt(year, month, day) {
        Some(d) => d,
        None => return 0,
    };

    let current_year = now.year();
    let birth_year = birth_date.year();

    // Calculate base age
    let mut age = current_year - birth_year;

    // Check if birthday hasn't occurred this year yet
    let this_year_birthday = NaiveDate::from_ymd_opt(current_year, month, day);

    if let Some(birthday_this_year) = this_year_birthday {
        let current_date = now.date_naive();
        if current_date < birthday_this_year {
            age -= 1;
        }
    }

    // Handle future birthdays (should return 0 or non-negative)
    if age < 0 {
        return 0;
    }

    age as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birthday_returns_positive_age() {
        // Test with a date far in the past
        let age = umt_birthday(1990, 1, 1, Some(0));
        assert!(age > 30);
    }

    #[test]
    fn test_birthday_future_date_returns_zero() {
        // Test with a date in the future
        let age = umt_birthday(2100, 1, 1, Some(0));
        assert_eq!(age, 0);
    }

    #[test]
    fn test_birthday_invalid_date_returns_zero() {
        // Test with invalid date (Feb 30)
        let age = umt_birthday(2000, 2, 30, Some(0));
        assert_eq!(age, 0);
    }

    #[test]
    fn test_birthday_with_timezone() {
        let age_jst = umt_birthday(1990, 1, 1, Some(9));
        let age_utc = umt_birthday(1990, 1, 1, Some(0));
        // Ages should be the same or differ by at most 1 (depending on date)
        assert!(age_jst.abs_diff(age_utc) <= 1);
    }

    #[test]
    fn test_birthday_default_timezone() {
        let age_default = umt_birthday(1990, 1, 1, None);
        let age_jst = umt_birthday(1990, 1, 1, Some(9));
        assert_eq!(age_default, age_jst);
    }
}
