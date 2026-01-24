//! Birthday and age calculation utilities.

use crate::date::now::umt_now;
use chrono::{Datelike, NaiveDate};

/// Calculates age based on birthdate.
///
/// # Arguments
///
/// * `year` - Birth year.
/// * `month` - Birth month (1-12).
/// * `day` - Birth day.
/// * `time_difference` - Time difference from UTC in hours (default: 9 for JST).
///
/// # Returns
///
/// The age in years. Returns 0 for future birthdates.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_birthday;
///
/// // Calculate age for someone born on Jan 1, 2000
/// let age = umt_birthday(2000, 1, 1, 9);
/// // Age will depend on current date
/// assert!(age >= 0);
/// ```
pub fn umt_birthday(year: i32, month: u32, day: u32, time_difference: i32) -> i32 {
    let birthday_date = match NaiveDate::from_ymd_opt(year, month, day) {
        Some(d) => d,
        None => return 0,
    };

    let now_time = umt_now(time_difference);
    let current_year = now_time.year();
    let birth_year = birthday_date.year();

    // Calculate base age
    let mut age = current_year - birth_year;

    // Check if birthday hasn't occurred this year yet
    let this_year_birthday = NaiveDate::from_ymd_opt(current_year, month, day);

    if let Some(this_year_bday) = this_year_birthday {
        let now_date = NaiveDate::from_ymd_opt(
            now_time.year(),
            now_time.month(),
            now_time.day(),
        );

        if let Some(current_date) = now_date {
            if current_date < this_year_bday {
                age -= 1;
            }
        }
    }

    // Handle future birthdays (should return 0 or non-negative)
    if age < 0 {
        return 0;
    }

    age
}

/// Calculates age based on birthdate using default timezone (JST/UTC+9).
///
/// # Arguments
///
/// * `year` - Birth year.
/// * `month` - Birth month (1-12).
/// * `day` - Birth day.
///
/// # Returns
///
/// The age in years. Returns 0 for future birthdates.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_birthday_jst;
///
/// // Calculate age for someone born on Jan 1, 2000
/// let age = umt_birthday_jst(2000, 1, 1);
/// assert!(age >= 0);
/// ```
#[inline]
pub fn umt_birthday_jst(year: i32, month: u32, day: u32) -> i32 {
    umt_birthday(year, month, day, 9)
}

/// Calculates age at a specific date.
///
/// # Arguments
///
/// * `birth_year` - Birth year.
/// * `birth_month` - Birth month (1-12).
/// * `birth_day` - Birth day.
/// * `at_year` - The year to calculate age at.
/// * `at_month` - The month to calculate age at.
/// * `at_day` - The day to calculate age at.
///
/// # Returns
///
/// The age in years at the specified date. Returns 0 for dates before birth.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_birthday_at;
///
/// // Calculate age on April 4, 2025 for someone born on Jan 1, 2000
/// let age = umt_birthday_at(2000, 1, 1, 2025, 4, 4);
/// assert_eq!(age, 25);
///
/// // Birthday hasn't occurred yet in the year
/// let age = umt_birthday_at(2000, 12, 31, 2025, 4, 4);
/// assert_eq!(age, 24);
/// ```
pub fn umt_birthday_at(
    birth_year: i32,
    birth_month: u32,
    birth_day: u32,
    at_year: i32,
    at_month: u32,
    at_day: u32,
) -> i32 {
    let birthday_date = match NaiveDate::from_ymd_opt(birth_year, birth_month, birth_day) {
        Some(d) => d,
        None => return 0,
    };

    let at_date = match NaiveDate::from_ymd_opt(at_year, at_month, at_day) {
        Some(d) => d,
        None => return 0,
    };

    // Calculate base age
    let mut age = at_year - birth_year;

    // Check if birthday hasn't occurred yet at the specified date
    let birthday_this_year = NaiveDate::from_ymd_opt(at_year, birth_month, birth_day);

    if let Some(bday) = birthday_this_year {
        if at_date < bday {
            age -= 1;
        }
    } else {
        // Handle leap year birthday (Feb 29)
        // If Feb 29 birthday and at_date year is not a leap year,
        // consider birthday as Feb 28
        if birth_month == 2 && birth_day == 29 {
            if let Some(feb28) = NaiveDate::from_ymd_opt(at_year, 2, 28) {
                if at_date < feb28 {
                    age -= 1;
                }
            }
        }
    }

    // Handle future dates (before birth)
    if age < 0 || at_date < birthday_date {
        return 0;
    }

    age
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birthday_at_past_birthday_this_year() {
        // Someone born on January 1, 2000 would be 25 on April 4, 2025
        let age = umt_birthday_at(2000, 1, 1, 2025, 4, 4);
        assert_eq!(age, 25);
    }

    #[test]
    fn test_birthday_at_future_birthday_this_year() {
        // Someone born on December 31, 2000 would still be 24 on April 4, 2025
        let age = umt_birthday_at(2000, 12, 31, 2025, 4, 4);
        assert_eq!(age, 24);
    }

    #[test]
    fn test_birthday_at_exact_birthday() {
        // On the exact birthday
        let age = umt_birthday_at(2000, 4, 4, 2025, 4, 4);
        assert_eq!(age, 25);
    }

    #[test]
    fn test_birthday_at_day_before_birthday() {
        // Day before birthday
        let age = umt_birthday_at(2000, 4, 4, 2025, 4, 3);
        assert_eq!(age, 24);
    }

    #[test]
    fn test_birthday_at_future_birthdate() {
        // Birth date is in the future
        let age = umt_birthday_at(2030, 1, 1, 2025, 4, 4);
        assert_eq!(age, 0);
    }

    #[test]
    fn test_birthday_at_newborn() {
        // Same day as birth
        let age = umt_birthday_at(2025, 4, 4, 2025, 4, 4);
        assert_eq!(age, 0);
    }

    #[test]
    fn test_birthday_at_first_birthday() {
        let age = umt_birthday_at(2024, 4, 4, 2025, 4, 4);
        assert_eq!(age, 1);
    }

    #[test]
    fn test_birthday_leap_year() {
        // Someone born on Feb 29
        let age = umt_birthday_at(2000, 2, 29, 2025, 3, 1);
        assert_eq!(age, 25);
    }

    #[test]
    fn test_birthday_leap_year_before_feb28() {
        // Someone born on Feb 29, checked on Feb 27 of non-leap year
        let age = umt_birthday_at(2000, 2, 29, 2025, 2, 27);
        assert_eq!(age, 24);
    }

    #[test]
    fn test_birthday_invalid_date() {
        // Invalid birth date
        let age = umt_birthday_at(2000, 2, 30, 2025, 4, 4);
        assert_eq!(age, 0);
    }

    #[test]
    fn test_birthday_jst() {
        // Just verify it doesn't panic and returns a reasonable value
        let age = umt_birthday_jst(2000, 1, 1);
        assert!(age >= 0);
    }

    #[test]
    fn test_birthday_with_timezone() {
        // Just verify it doesn't panic and returns a reasonable value
        let age_jst = umt_birthday(2000, 1, 1, 9);
        let age_utc = umt_birthday(2000, 1, 1, 0);

        // Both should be non-negative
        assert!(age_jst >= 0);
        assert!(age_utc >= 0);

        // Ages should be close (within 1 year due to timezone differences)
        assert!((age_jst - age_utc).abs() <= 1);
    }
}
