use super::now_simple::{umt_new_date, umt_now_simple, DateTime};

/// Properties for specifying a date.
#[derive(Debug, Clone, Default)]
pub struct DateProperties {
    /// Year (optional, defaults to current year)
    pub year: Option<i32>,
    /// Month 1-12 (optional, defaults to current month)
    pub mon: Option<u32>,
    /// Day 1-31 (optional, defaults to current day)
    pub day: Option<u32>,
}

impl DateProperties {
    /// Creates a new DateProperties with the given values.
    pub fn new(year: Option<i32>, mon: Option<u32>, day: Option<u32>) -> Self {
        DateProperties { year, mon, day }
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

/// Get the day of the week for a given date.
///
/// Returns a number representing the day of the week (0 = Sunday, 6 = Saturday).
///
/// # Arguments
///
/// * `properties` - Date properties (year, month, day). Defaults to current date if None.
/// * `time_difference` - Time zone difference in hours (default: 9 for JST)
///
/// # Returns
///
/// Day of the week as u32 (0 = Sunday, 6 = Saturday)
///
/// # Examples
///
/// ```
/// use umt_rust::simple::date::{umt_day_of_week_simple, DateProperties};
///
/// // Get day of week for January 2, 2022 (Sunday)
/// let day = umt_day_of_week_simple(Some(DateProperties::new(Some(2022), Some(1), Some(2))), 9);
/// assert_eq!(day, 0);
/// ```
#[inline]
pub fn umt_day_of_week_simple(properties: Option<DateProperties>, time_difference: i32) -> u32 {
    let now = umt_now_simple(time_difference);

    match properties {
        Some(props) => {
            let year = props.year.unwrap_or(now.year);
            let month = props.mon.unwrap_or(now.month);
            let day = props.day.unwrap_or(now.day);

            umt_new_date(year, month, day).day_of_week
        }
        None => now.day_of_week,
    }
}

/// Get the day of the week for a date string.
///
/// Supports formats: "YYYY-MM-DD", "YYYY:MM:DD", "YYYY/MM/DD"
///
/// # Arguments
///
/// * `date_str` - Date string in one of the supported formats
/// * `time_difference` - Time zone difference in hours (default: 9 for JST)
///
/// # Returns
///
/// Day of the week as u32 (0 = Sunday, 6 = Saturday)
///
/// # Examples
///
/// ```
/// use umt_rust::simple::date::umt_day_of_week_simple_str;
///
/// // Sunday
/// assert_eq!(umt_day_of_week_simple_str("2022-01-02", 9), 0);
/// // Monday
/// assert_eq!(umt_day_of_week_simple_str("2022/01/03", 9), 1);
/// // Tuesday
/// assert_eq!(umt_day_of_week_simple_str("2022:01:04", 9), 2);
/// ```
#[inline]
pub fn umt_day_of_week_simple_str(date_str: &str, time_difference: i32) -> u32 {
    if let Some((year, month, day)) = parse_date_string(date_str) {
        umt_new_date(year, month, day).day_of_week
    } else {
        // Fall back to current day if parsing fails
        umt_now_simple(time_difference).day_of_week
    }
}

/// Get the day of the week for a DateTime.
///
/// # Arguments
///
/// * `date` - DateTime object
/// * `time_difference` - Time zone difference in hours (used for timezone adjustment)
///
/// # Returns
///
/// Day of the week as u32 (0 = Sunday, 6 = Saturday)
///
/// # Examples
///
/// ```
/// use umt_rust::simple::date::{umt_day_of_week_simple_datetime, umt_new_date};
///
/// let date = umt_new_date(2022, 1, 2);
/// assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 0); // Sunday
/// ```
#[inline]
pub fn umt_day_of_week_simple_datetime(date: &DateTime, _time_difference: i32) -> u32 {
    // The DateTime already has day_of_week calculated
    // For JavaScript Date compatibility, we need to recalculate based on the actual date components
    umt_new_date(date.year, date.month, date.day).day_of_week
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sunday() {
        assert_eq!(umt_day_of_week_simple_str("2022-01-02", 9), 0);
    }

    #[test]
    fn test_monday() {
        assert_eq!(umt_day_of_week_simple_str("2022/01/03", 9), 1);
    }

    #[test]
    fn test_tuesday() {
        assert_eq!(umt_day_of_week_simple_str("2022:01:04", 9), 2);
    }

    #[test]
    fn test_wednesday() {
        let props = DateProperties::new(Some(2022), Some(1), Some(5));
        assert_eq!(umt_day_of_week_simple(Some(props), 9), 3);
    }

    #[test]
    fn test_thursday() {
        assert_eq!(umt_day_of_week_simple_str("2022-01-06", 9), 4);
    }

    #[test]
    fn test_friday() {
        assert_eq!(umt_day_of_week_simple_str("2022-01-07", 9), 5);
    }

    #[test]
    fn test_saturday() {
        assert_eq!(umt_day_of_week_simple_str("2022-01-08", 9), 6);
    }

    #[test]
    fn test_with_datetime() {
        let date = umt_new_date(2022, 1, 2);
        assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 0); // Sunday
    }

    #[test]
    fn test_with_datetime_monday() {
        let date = umt_new_date(2022, 1, 3);
        assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 1); // Monday
    }

    #[test]
    fn test_with_datetime_saturday() {
        let date = umt_new_date(2022, 1, 8);
        assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 6); // Saturday
    }

    #[test]
    fn test_with_slash_delimiter() {
        assert_eq!(umt_day_of_week_simple_str("2022/01/02", 9), 0);
        assert_eq!(umt_day_of_week_simple_str("2022/01/03", 9), 1);
    }

    #[test]
    fn test_with_colon_delimiter() {
        assert_eq!(umt_day_of_week_simple_str("2022:01:02", 9), 0);
        assert_eq!(umt_day_of_week_simple_str("2022:01:03", 9), 1);
    }

    #[test]
    fn test_known_dates() {
        // Additional verification with known dates
        // 2024-01-01 is Monday
        assert_eq!(umt_day_of_week_simple_str("2024-01-01", 9), 1);
        // 2024-12-25 is Wednesday
        assert_eq!(umt_day_of_week_simple_str("2024-12-25", 9), 3);
        // 2000-01-01 was Saturday
        assert_eq!(umt_day_of_week_simple_str("2000-01-01", 9), 6);
    }

    #[test]
    fn test_with_default_properties() {
        // When called with None, should return current day of week
        let day = umt_day_of_week_simple(None, 9);
        assert!(day < 7);
    }

    #[test]
    fn test_partial_properties() {
        // Test with partial properties - only year specified
        let now = umt_now_simple(9);
        let props = DateProperties::new(Some(2022), None, None);
        let day = umt_day_of_week_simple(Some(props), 9);
        // Should use 2022 with current month and day
        assert!(day < 7);
    }
}
