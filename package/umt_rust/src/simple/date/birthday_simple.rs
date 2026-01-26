use super::now_simple::{DateTime, umt_new_date, umt_now_simple};

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
    umt_birthday(
        birthdays.year,
        birthdays.month,
        birthdays.day,
        time_difference,
    )
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
    umt_birthday(
        birthdays.year,
        birthdays.mon,
        birthdays.day,
        time_difference,
    )
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
