use std::time::{SystemTime, UNIX_EPOCH};

/// One hour in milliseconds
const ONE_HOUR_MS: i64 = 3600 * 1000;

/// Represents a date and time with timezone support.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTime {
    /// Year (e.g., 2024)
    pub year: i32,
    /// Month (1-12)
    pub month: u32,
    /// Day of month (1-31)
    pub day: u32,
    /// Hour (0-23)
    pub hour: u32,
    /// Minute (0-59)
    pub minute: u32,
    /// Second (0-59)
    pub second: u32,
    /// Millisecond (0-999)
    pub millisecond: u32,
    /// Day of week (0 = Sunday, 6 = Saturday)
    pub day_of_week: u32,
    /// Timestamp in milliseconds since Unix epoch (adjusted for timezone)
    pub timestamp_ms: i64,
}

impl DateTime {
    /// Creates a new DateTime from a timestamp and timezone offset.
    ///
    /// # Arguments
    ///
    /// * `timestamp_ms` - Timestamp in milliseconds since Unix epoch
    /// * `time_difference` - Hours offset from UTC
    pub fn from_timestamp(timestamp_ms: i64, time_difference: i32) -> Self {
        let adjusted_ms = timestamp_ms + (time_difference as i64 * ONE_HOUR_MS);

        // Calculate date components from timestamp
        let total_days = adjusted_ms.div_euclid(86400 * 1000);
        let day_ms = adjusted_ms.rem_euclid(86400 * 1000) as u32;

        let hour = day_ms / (3600 * 1000);
        let minute = (day_ms % (3600 * 1000)) / (60 * 1000);
        let second = (day_ms % (60 * 1000)) / 1000;
        let millisecond = day_ms % 1000;

        // Calculate day of week (Jan 1, 1970 was Thursday = 4)
        let day_of_week = ((total_days + 4) % 7) as u32;

        // Calculate year, month, day using civil calendar algorithm
        let (year, month, day) = days_to_ymd(total_days);

        DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond,
            day_of_week,
            timestamp_ms: adjusted_ms,
        }
    }

    /// Returns the UTC hours component.
    pub fn get_utc_hours(&self) -> u32 {
        self.hour
    }

    /// Returns the year.
    pub fn get_full_year(&self) -> i32 {
        self.year
    }

    /// Returns the month (0-11, JavaScript-style).
    pub fn get_month(&self) -> u32 {
        self.month - 1
    }

    /// Returns the day of month (1-31).
    pub fn get_date(&self) -> u32 {
        self.day
    }

    /// Returns the day of week (0 = Sunday, 6 = Saturday).
    pub fn get_day(&self) -> u32 {
        self.day_of_week
    }
}

/// Converts days since Unix epoch to year, month, day.
fn days_to_ymd(days: i64) -> (i32, u32, u32) {
    // Algorithm from Howard Hinnant's date algorithms
    // http://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u32; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // year of era [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // day of year [0, 365]
    let mp = (5 * doy + 2) / 153; // month [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // day [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // month [1, 12]
    let y = if m <= 2 { y + 1 } else { y };

    (y as i32, m, d)
}

/// Converts year, month, day to days since Unix epoch.
fn ymd_to_days(year: i32, month: u32, day: u32) -> i64 {
    let y = if month <= 2 {
        year as i64 - 1
    } else {
        year as i64
    };
    let m = month as i64;
    let d = day as i64;

    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u32; // year of era [0, 399]
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1; // day of year [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy as u32; // day of era [0, 146096]

    era * 146097 + doe as i64 - 719468
}

/// Get the current time with a specified UTC offset.
///
/// Returns a `DateTime` representing the current date and time adjusted
/// for the specified timezone offset.
///
/// # Arguments
///
/// * `time_difference` - Hours offset from UTC (default: 9 for Japan Standard Time)
///
/// # Returns
///
/// `DateTime` object representing the current date and time
///
/// # Examples
///
/// ```
/// use umt_rust::simple::date::umt_now_simple;
///
/// // Get current time in JST (UTC+9)
/// let now = umt_now_simple(9);
/// println!("Current year: {}", now.year);
/// ```
///
/// ```
/// use umt_rust::simple::date::umt_now_simple;
///
/// // Get current time in UTC
/// let now_utc = umt_now_simple(0);
/// ```
#[inline]
pub fn umt_now_simple(time_difference: i32) -> DateTime {
    let timestamp_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64;

    DateTime::from_timestamp(timestamp_ms, time_difference)
}

/// Get the current time in JST (UTC+9).
///
/// This is a convenience function that returns the current time
/// with Japan Standard Time offset (UTC+9).
///
/// # Returns
///
/// `DateTime` object representing the current date and time in JST
///
/// # Examples
///
/// ```
/// use umt_rust::simple::date::umt_now_simple_jst;
///
/// let now_jst = umt_now_simple_jst();
/// println!("Current hour in JST: {}", now_jst.hour);
/// ```
#[inline]
pub fn umt_now_simple_jst() -> DateTime {
    umt_now_simple(9)
}

/// Creates a DateTime from year, month (1-12), and day components.
///
/// # Arguments
///
/// * `year` - The year
/// * `month` - The month (1-12)
/// * `day` - The day of month (1-31)
///
/// # Returns
///
/// `DateTime` object representing the specified date at midnight UTC
pub fn umt_new_date(year: i32, month: u32, day: u32) -> DateTime {
    let days = ymd_to_days(year, month, day);
    let timestamp_ms = days * 86400 * 1000;
    DateTime::from_timestamp(timestamp_ms, 0)
}
