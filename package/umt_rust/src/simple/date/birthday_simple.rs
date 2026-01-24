use super::now_simple::{umt_new_date, umt_now_simple, DateTime};

/// Properties for specifying a birthday.
#[derive(Debug, Clone)]
pub struct BirthdayProperties {
    /// Birth year
    pub year: i32,
    /// Birth month (1-12)
    pub mon: u32,
    /// Birth day (1-31)
    pub day: u32,
}

impl BirthdayProperties {
    /// Creates a new BirthdayProperties with the given values.
    pub fn new(year: i32, mon: u32, day: u32) -> Self {
        BirthdayProperties { year, mon, day }
    }
}

/// Parses a date string in the format "YYYY-MM-DD", "YYYY:MM:DD", or "YYYY/MM/DD".
///
/// # Arguments
///
/// * `date_str` - Date string to parse
///
/// # Returns
///
/// Optional tuple of (year, month, day) if parsing succeeds
fn parse_date_string(date_str: &str) -> Option<(i32, u32, u32)> {
    let parts: Vec<&str> = if date_str.contains(':') {
        date_str.split(':').collect()
    } else if date_str.contains('/') {
        date_str.split('/').collect()
    } else {
        date_str.split('-').collect()
    };

    if parts.len() == 3 {
        let year = parts[0].parse::<i32>().ok()?;
        let month = parts[1].parse::<u32>().ok()?;
        let day = parts[2].parse::<u32>().ok()?;
        Some((year, month, day))
    } else {
        None
    }
}

/// Calculate age based on birthdate.
///
/// # Arguments
///
/// * `year` - Birth year
/// * `mon` - Birth month (1-12)
/// * `day` - Birth day (1-31)
/// * `time_difference` - Time zone difference in hours from UTC (default: 9 for JST)
///
/// # Returns
///
/// Age in years (0 if birthday is in the future)
///
/// # Examples
///
/// ```
/// use umt_rust::simple::date::umt_birthday;
///
/// // Calculate age for someone born on January 1, 2000
/// let age = umt_birthday(2000, 1, 1, 9);
/// // Age will depend on current date
/// assert!(age >= 0);
/// ```
#[inline]
pub fn umt_birthday(year: i32, mon: u32, day: u32, time_difference: i32) -> i32 {
    let birthday_date = umt_new_date(year, mon, day);
    let now_time = umt_now_simple(time_difference);

    let current_year = now_time.year;
    let birth_year = birthday_date.year;

    // Calculate base age
    let mut age = current_year - birth_year;

    // Check if birthday hasn't occurred this year yet
    // Using month and day comparison instead of timestamp for accuracy
    if now_time.month < birthday_date.month
        || (now_time.month == birthday_date.month && now_time.day < birthday_date.day)
    {
        age -= 1;
    }

    // Handle future birthdays (should return 0 or non-negative)
    if age < 0 {
        return 0;
    }

    age
}

/// Calculate age from a date string.
///
/// Supports formats: "YYYY-MM-DD", "YYYY:MM:DD", "YYYY/MM/DD"
///
/// # Arguments
///
/// * `birthdays` - Birthday string in one of the supported formats
/// * `time_difference` - Time zone difference in hours from UTC (default: 9 for JST)
///
/// # Returns
///
/// Age in years (0 if birthday is in the future or parsing fails)
///
/// # Examples
///
/// ```
/// use umt_rust::simple::date::umt_birthday_simple_str;
///
/// let age = umt_birthday_simple_str("2000-01-01", 9);
/// assert!(age >= 0);
///
/// let age = umt_birthday_simple_str("2000:01:01", 9);
/// assert!(age >= 0);
///
/// let age = umt_birthday_simple_str("2000/01/01", 9);
/// assert!(age >= 0);
/// ```
#[inline]
pub fn umt_birthday_simple_str(birthdays: &str, time_difference: i32) -> i32 {
    if let Some((year, mon, day)) = parse_date_string(birthdays) {
        umt_birthday(year, mon, day, time_difference)
    } else {
        0
    }
}

/// Calculate age from a DateTime object.
///
/// Note: For JavaScript Date compatibility, month is 0-indexed (0 = January).
///
/// # Arguments
///
/// * `birthdays` - Birthday as DateTime object
/// * `time_difference` - Time zone difference in hours from UTC (default: 9 for JST)
///
/// # Returns
///
/// Age in years (0 if birthday is in the future)
///
/// # Examples
///
/// ```
/// use umt_rust::simple::date::{umt_birthday_simple_datetime, umt_new_date};
///
/// let birthday = umt_new_date(2000, 1, 1);
/// let age = umt_birthday_simple_datetime(&birthday, 9);
/// assert!(age >= 0);
/// ```
#[inline]
pub fn umt_birthday_simple_datetime(birthdays: &DateTime, time_difference: i32) -> i32 {
    // Note: For JavaScript Date compatibility, getMonth() returns 0-indexed month
    // But our DateTime uses 1-indexed month internally
    umt_birthday(birthdays.year, birthdays.month, birthdays.day, time_difference)
}

/// Calculate age from BirthdayProperties.
///
/// # Arguments
///
/// * `birthdays` - Birthday properties containing year, month (1-12), and day
/// * `time_difference` - Time zone difference in hours from UTC (default: 9 for JST)
///
/// # Returns
///
/// Age in years (0 if birthday is in the future)
///
/// # Examples
///
/// ```
/// use umt_rust::simple::date::{umt_birthday_simple_props, BirthdayProperties};
///
/// let props = BirthdayProperties::new(2000, 1, 1);
/// let age = umt_birthday_simple_props(&props, 9);
/// assert!(age >= 0);
/// ```
#[inline]
pub fn umt_birthday_simple_props(birthdays: &BirthdayProperties, time_difference: i32) -> i32 {
    umt_birthday(birthdays.year, birthdays.mon, birthdays.day, time_difference)
}

/// Calculate age from a date string with default timezone (JST).
///
/// This is a convenience function that uses JST (UTC+9) as the default timezone.
///
/// # Arguments
///
/// * `birthdays` - Birthday string in format "YYYY-MM-DD", "YYYY:MM:DD", or "YYYY/MM/DD"
///
/// # Returns
///
/// Age in years (0 if birthday is in the future)
#[inline]
pub fn umt_birthday_simple(birthdays: &str) -> i32 {
    umt_birthday_simple_str(birthdays, 9)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests assume the current date is around 2024-2026
    // Adjust expected values if running tests in a different time period

    #[test]
    fn test_basic_age_calculation_hyphen() {
        let age = umt_birthday_simple_str("2000-01-01", 9);
        // Should be around 24-26 depending on current date
        assert!(age >= 24 && age <= 30);
    }

    #[test]
    fn test_basic_age_calculation_colon() {
        let age = umt_birthday_simple_str("2000:01:01", 9);
        assert!(age >= 24 && age <= 30);
    }

    #[test]
    fn test_basic_age_calculation_slash() {
        let age = umt_birthday_simple_str("2000/01/01", 9);
        assert!(age >= 24 && age <= 30);
    }

    #[test]
    fn test_future_birthday() {
        // Far future birthday should return 0
        let age = umt_birthday_simple_str("2100-01-01", 9);
        assert_eq!(age, 0);
    }

    #[test]
    fn test_very_old_birthday() {
        let age = umt_birthday_simple_str("1900-01-01", 9);
        // Should be around 124-126
        assert!(age >= 124 && age <= 130);
    }

    #[test]
    fn test_recent_birthday() {
        // Someone born 1 year ago (approximately)
        let now = umt_now_simple(9);
        let birth_year = now.year - 1;
        let date_str = format!("{}-01-01", birth_year);
        let age = umt_birthday_simple_str(&date_str, 9);
        // Should be 0 or 1 depending on whether birthday has passed
        assert!(age <= 2);
    }

    #[test]
    fn test_leap_year() {
        let age = umt_birthday_simple_str("2000-02-29", 9);
        assert!(age >= 24 && age <= 30);
    }

    #[test]
    fn test_with_datetime() {
        let birthday = umt_new_date(2000, 1, 1);
        let age = umt_birthday_simple_datetime(&birthday, 9);
        assert!(age >= 24 && age <= 30);
    }

    #[test]
    fn test_with_properties() {
        let props = BirthdayProperties::new(2000, 1, 1);
        let age = umt_birthday_simple_props(&props, 9);
        assert!(age >= 24 && age <= 30);
    }

    #[test]
    fn test_different_timezones() {
        let age_utc = umt_birthday_simple_str("2000-01-01", 0);
        let age_jst = umt_birthday_simple_str("2000-01-01", 9);
        // Ages should be the same or differ by at most 1 (due to timezone boundary)
        assert!((age_utc - age_jst).abs() <= 1);
    }

    #[test]
    fn test_convenience_function() {
        let age = umt_birthday_simple("2000-01-01");
        assert!(age >= 24 && age <= 30);
    }

    #[test]
    fn test_month_boundary() {
        let age = umt_birthday_simple_str("2000-12-31", 9);
        assert!(age >= 23 && age <= 30);
    }

    #[test]
    fn test_invalid_date_string() {
        // Invalid format should return 0
        let age = umt_birthday_simple_str("invalid", 9);
        assert_eq!(age, 0);
    }

    #[test]
    fn test_age_is_non_negative() {
        // Test with various dates to ensure age is never negative
        let dates = vec![
            "2100-01-01",
            "2050-06-15",
            "1900-01-01",
            "2000-01-01",
        ];

        for date in dates {
            let age = umt_birthday_simple_str(date, 9);
            assert!(age >= 0, "Age should be non-negative for date {}", date);
        }
    }
}
