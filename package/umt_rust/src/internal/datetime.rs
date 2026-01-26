use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// ---- Duration ----

/// A duration type representing a span of time in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UmtDuration {
    millis: i64,
}

impl UmtDuration {
    pub fn milliseconds(ms: i64) -> Self {
        UmtDuration { millis: ms }
    }

    pub fn seconds(s: i64) -> Self {
        UmtDuration {
            millis: s * 1_000,
        }
    }

    pub fn hours(h: i64) -> Self {
        UmtDuration {
            millis: h * 3_600_000,
        }
    }

    pub fn days(d: i64) -> Self {
        UmtDuration {
            millis: d * 86_400_000,
        }
    }

    pub fn num_hours(&self) -> i64 {
        self.millis / 3_600_000
    }

    pub fn num_milliseconds(&self) -> i64 {
        self.millis
    }
}

// ---- NaiveDate ----

/// A date without timezone information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UmtNaiveDate {
    year: i32,
    month: u32,
    day: u32,
}

impl UmtNaiveDate {
    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<Self> {
        if month < 1 || month > 12 || day < 1 || day > days_in_month(year, month) {
            return None;
        }
        Some(UmtNaiveDate { year, month, day })
    }

    /// Parse from "YYYY-MM-DD" format
    pub fn parse_from_str(s: &str, _fmt: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return None;
        }
        let year: i32 = parts[0].parse().ok()?;
        let month: u32 = parts[1].parse().ok()?;
        let day: u32 = parts[2].parse().ok()?;
        Self::from_ymd_opt(year, month, day)
    }

    pub fn year(&self) -> i32 {
        self.year
    }
    pub fn month(&self) -> u32 {
        self.month
    }
    pub fn day(&self) -> u32 {
        self.day
    }

    /// Days since Sunday (0=Sun, 6=Sat)
    pub fn weekday_num_from_sunday(&self) -> u32 {
        let epoch_days = days_from_civil(self.year, self.month, self.day);
        weekday_from_epoch_days(epoch_days)
    }

    /// Convert to epoch days
    pub fn to_epoch_days(&self) -> i64 {
        days_from_civil(self.year, self.month, self.day)
    }
}

impl fmt::Display for UmtNaiveDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

// ---- NaiveTime ----

/// A time without timezone information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UmtNaiveTime {
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
}

impl UmtNaiveTime {
    pub fn from_hms_milli_opt(hour: u32, minute: u32, second: u32, milli: u32) -> Option<Self> {
        if hour > 23 || minute > 59 || second > 59 || milli > 999 {
            return None;
        }
        Some(UmtNaiveTime {
            hour,
            minute,
            second,
            millisecond: milli,
        })
    }
}

// ---- DateTime ----

/// A UTC datetime type backed by a Unix timestamp in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UmtDateTime {
    timestamp_ms: i64,
}

impl UmtDateTime {
    /// Get the current UTC time.
    pub fn now() -> Self {
        let ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;
        UmtDateTime { timestamp_ms: ms }
    }

    /// Create from a Unix timestamp in milliseconds.
    pub fn from_timestamp_ms(ms: i64) -> Self {
        UmtDateTime { timestamp_ms: ms }
    }

    /// Create from date and time components (UTC).
    pub fn from_ymd_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Option<Self> {
        Self::from_ymd_hms_milli(year, month, day, hour, minute, second, 0)
    }

    /// Create from date and time components with milliseconds (UTC).
    pub fn from_ymd_hms_milli(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        milli: u32,
    ) -> Option<Self> {
        if month < 1 || month > 12 || day < 1 || day > days_in_month(year, month) {
            return None;
        }
        if hour > 23 || minute > 59 || second > 59 || milli > 999 {
            return None;
        }

        let epoch_days = days_from_civil(year, month, day);
        let ms = epoch_days * 86_400_000
            + hour as i64 * 3_600_000
            + minute as i64 * 60_000
            + second as i64 * 1_000
            + milli as i64;

        Some(UmtDateTime { timestamp_ms: ms })
    }

    /// Create from NaiveDate and NaiveTime
    pub fn from_naive(date: UmtNaiveDate, time: UmtNaiveTime) -> Self {
        let epoch_days = days_from_civil(date.year, date.month, date.day);
        let ms = epoch_days * 86_400_000
            + time.hour as i64 * 3_600_000
            + time.minute as i64 * 60_000
            + time.second as i64 * 1_000
            + time.millisecond as i64;
        UmtDateTime { timestamp_ms: ms }
    }

    /// Unix timestamp in seconds
    pub fn timestamp(&self) -> i64 {
        self.timestamp_ms.div_euclid(1000)
    }

    /// Unix timestamp in milliseconds
    pub fn timestamp_millis(&self) -> i64 {
        self.timestamp_ms
    }

    /// Extract date/time components
    fn components(&self) -> (i32, u32, u32, u32, u32, u32, u32) {
        let total_ms = self.timestamp_ms;

        // Compute the number of whole days and remaining ms
        let epoch_days = total_ms.div_euclid(86_400_000);
        let day_ms = total_ms.rem_euclid(86_400_000) as u64;

        let (year, month, day) = civil_from_days(epoch_days);

        let hour = (day_ms / 3_600_000) as u32;
        let minute = ((day_ms % 3_600_000) / 60_000) as u32;
        let second = ((day_ms % 60_000) / 1_000) as u32;
        let milli = (day_ms % 1_000) as u32;

        (year, month, day, hour, minute, second, milli)
    }

    pub fn year(&self) -> i32 {
        self.components().0
    }

    pub fn month(&self) -> u32 {
        self.components().1
    }

    pub fn day(&self) -> u32 {
        self.components().2
    }

    pub fn hour(&self) -> u32 {
        self.components().3
    }

    pub fn minute(&self) -> u32 {
        self.components().4
    }

    pub fn second(&self) -> u32 {
        self.components().5
    }

    /// Returns nanosecond component (always multiple of 1_000_000 since we store ms)
    pub fn nanosecond(&self) -> u32 {
        self.components().6 * 1_000_000
    }

    /// Day of week from Sunday (0=Sun, 6=Sat)
    pub fn weekday_num_from_sunday(&self) -> u32 {
        let epoch_days = self.timestamp_ms.div_euclid(86_400_000);
        weekday_from_epoch_days(epoch_days)
    }

    /// Get the date part as UmtNaiveDate
    pub fn date_naive(&self) -> UmtNaiveDate {
        let (y, m, d, _, _, _, _) = self.components();
        UmtNaiveDate {
            year: y,
            month: m,
            day: d,
        }
    }

    /// Add a duration
    pub fn add_duration(self, duration: UmtDuration) -> Self {
        UmtDateTime {
            timestamp_ms: self.timestamp_ms + duration.millis,
        }
    }

    /// Subtract a duration
    pub fn sub_duration(self, duration: UmtDuration) -> Self {
        UmtDateTime {
            timestamp_ms: self.timestamp_ms - duration.millis,
        }
    }

    /// Signed duration between self and another datetime
    pub fn signed_duration_since(&self, other: &Self) -> UmtDuration {
        UmtDuration {
            millis: self.timestamp_ms - other.timestamp_ms,
        }
    }

    /// Format with a pattern string (e.g., "%Y-%m-%d")
    pub fn format_str(&self, fmt: &str) -> String {
        let (y, m, d, h, min, s, _ms) = self.components();
        fmt.replace("%Y", &format!("{:04}", y))
            .replace("%m", &format!("{:02}", m))
            .replace("%d", &format!("{:02}", d))
            .replace("%H", &format!("{:02}", h))
            .replace("%M", &format!("{:02}", min))
            .replace("%S", &format!("{:02}", s))
    }
}

impl std::ops::Add<UmtDuration> for UmtDateTime {
    type Output = UmtDateTime;
    fn add(self, rhs: UmtDuration) -> UmtDateTime {
        self.add_duration(rhs)
    }
}

impl std::ops::Sub<UmtDuration> for UmtDateTime {
    type Output = UmtDateTime;
    fn sub(self, rhs: UmtDuration) -> UmtDateTime {
        self.sub_duration(rhs)
    }
}

impl fmt::Display for UmtDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d, h, min, s, _ms) = self.components();
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            y, m, d, h, min, s
        )
    }
}

// ---- Calendar math (Howard Hinnant's algorithms) ----

pub fn is_leap_year(y: i32) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

pub fn days_in_month(y: i32, m: u32) -> u32 {
    match m {
        1 => 31,
        2 => {
            if is_leap_year(y) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0,
    }
}

/// Convert civil date to days since Unix epoch (1970-01-01)
pub fn days_from_civil(y: i32, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y as i64 - 1 } else { y as i64 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u64; // [0, 399]
    let doy = (153 * (if m > 2 { m as u64 - 3 } else { m as u64 + 9 }) + 2) / 5 + d as u64 - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era * 146097 + doe as i64 - 719468
}

/// Convert days since Unix epoch to civil date (year, month, day)
pub fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u32, d)
}

/// Day of week from epoch days (0=Sunday, 6=Saturday)
pub fn weekday_from_epoch_days(epoch_days: i64) -> u32 {
    ((epoch_days % 7 + 11) % 7) as u32
}

/// Get the local timezone offset in seconds from UTC.
#[cfg(unix)]
pub fn local_utc_offset_seconds() -> i32 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    #[repr(C)]
    struct Tm {
        tm_sec: i32,
        tm_min: i32,
        tm_hour: i32,
        tm_mday: i32,
        tm_mon: i32,
        tm_year: i32,
        tm_wday: i32,
        tm_yday: i32,
        tm_isdst: i32,
        tm_gmtoff: std::ffi::c_long,
        tm_zone: *const std::ffi::c_char,
    }

    unsafe extern "C" {
        fn localtime_r(time: *const i64, result: *mut Tm) -> *mut Tm;
    }

    let mut tm = std::mem::MaybeUninit::<Tm>::zeroed();
    unsafe {
        let result = localtime_r(&timestamp, tm.as_mut_ptr());
        if result.is_null() {
            return 0;
        }
        tm.assume_init().tm_gmtoff as i32
    }
}

#[cfg(not(unix))]
pub fn local_utc_offset_seconds() -> i32 {
    0 // Default to UTC on non-Unix platforms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_from_civil() {
        // Unix epoch
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        // Day after epoch
        assert_eq!(days_from_civil(1970, 1, 2), 1);
        // 2025-01-01
        assert_eq!(days_from_civil(2025, 1, 1), 20089);
    }

    #[test]
    fn test_civil_from_days() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(1), (1970, 1, 2));
        assert_eq!(civil_from_days(20089), (2025, 1, 1));
    }

    #[test]
    fn test_roundtrip() {
        for &(y, m, d) in &[
            (1970, 1, 1),
            (2000, 2, 29),
            (2025, 4, 4),
            (1969, 12, 31),
            (1, 1, 1),
        ] {
            let days = days_from_civil(y, m, d);
            assert_eq!(civil_from_days(days), (y, m, d));
        }
    }

    #[test]
    fn test_weekday() {
        // 1970-01-01 was Thursday (4)
        assert_eq!(weekday_from_epoch_days(0), 4);
        // 2025-01-01 was Wednesday (3)
        assert_eq!(weekday_from_epoch_days(days_from_civil(2025, 1, 1)), 3);
        // 2025-01-05 was Sunday (0)
        assert_eq!(weekday_from_epoch_days(days_from_civil(2025, 1, 5)), 0);
    }

    #[test]
    fn test_datetime_components() {
        let dt = UmtDateTime::from_ymd_hms(2025, 4, 4, 15, 30, 45).unwrap();
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 4);
        assert_eq!(dt.day(), 4);
        assert_eq!(dt.hour(), 15);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 45);
    }

    #[test]
    fn test_datetime_add_duration() {
        let dt = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let dt2 = dt + UmtDuration::days(1);
        assert_eq!(dt2.year(), 2025);
        assert_eq!(dt2.month(), 1);
        assert_eq!(dt2.day(), 2);
    }

    #[test]
    fn test_datetime_signed_duration() {
        let dt1 = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let dt2 = UmtDateTime::from_ymd_hms(2025, 1, 1, 9, 0, 0).unwrap();
        let duration = dt2.signed_duration_since(&dt1);
        assert_eq!(duration.num_hours(), 9);
    }

    #[test]
    fn test_leap_year() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2025));
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2025, 2), 28);
        assert_eq!(days_in_month(2025, 1), 31);
        assert_eq!(days_in_month(2025, 4), 30);
    }

    #[test]
    fn test_naive_date() {
        let d = UmtNaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert_eq!(d.year(), 2025);
        assert_eq!(d.month(), 1);
        assert_eq!(d.day(), 1);
        assert_eq!(d.weekday_num_from_sunday(), 3); // Wednesday
    }

    #[test]
    fn test_naive_date_invalid() {
        assert!(UmtNaiveDate::from_ymd_opt(2025, 13, 1).is_none());
        assert!(UmtNaiveDate::from_ymd_opt(2025, 2, 30).is_none());
    }

    #[test]
    fn test_format() {
        let dt = UmtDateTime::from_ymd_hms(2025, 4, 4, 15, 30, 45).unwrap();
        assert_eq!(dt.format_str("%Y-%m-%d"), "2025-04-04");
        assert_eq!(dt.format_str("%H:%M:%S"), "15:30:45");
    }
}
