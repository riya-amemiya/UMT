use super::now_simple::{DateTime, umt_new_date, umt_now_simple};

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
