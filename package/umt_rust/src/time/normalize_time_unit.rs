use std::fmt;

/// Time unit enumeration representing different time measurements.
///
/// Supports both long format (milliseconds, seconds, minutes, hours) and
/// short format (ms, s, m, h) for parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
}

impl TimeUnit {
    /// Returns the long format string representation of the time unit.
    ///
    /// # Returns
    ///
    /// A string slice with the long format name (e.g., "milliseconds", "seconds").
    #[inline]
    pub fn to_long(&self) -> &'static str {
        match self {
            TimeUnit::Milliseconds => "milliseconds",
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
        }
    }

    /// Returns the short format string representation of the time unit.
    ///
    /// # Returns
    ///
    /// A string slice with the short format name (e.g., "ms", "s", "m", "h").
    #[inline]
    pub fn to_short(&self) -> &'static str {
        match self {
            TimeUnit::Milliseconds => "ms",
            TimeUnit::Seconds => "s",
            TimeUnit::Minutes => "m",
            TimeUnit::Hours => "h",
        }
    }

    /// Returns the conversion rate to milliseconds.
    ///
    /// # Returns
    ///
    /// The number of milliseconds in one unit of this time type.
    #[inline]
    pub fn to_milliseconds_rate(&self) -> f64 {
        match self {
            TimeUnit::Milliseconds => 1.0,
            TimeUnit::Seconds => 1000.0,
            TimeUnit::Minutes => 60_000.0,
            TimeUnit::Hours => 3_600_000.0,
        }
    }

    /// Creates a TimeUnit from a string representation.
    ///
    /// Accepts both long format (milliseconds, seconds, minutes, hours) and
    /// short format (ms, s, m, h).
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice representing the time unit.
    ///
    /// # Returns
    ///
    /// `Some(TimeUnit)` if the string is a valid time unit, `None` otherwise.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<TimeUnit> {
        match s {
            "milliseconds" | "ms" => Some(TimeUnit::Milliseconds),
            "seconds" | "s" => Some(TimeUnit::Seconds),
            "minutes" | "m" => Some(TimeUnit::Minutes),
            "hours" | "h" => Some(TimeUnit::Hours),
            _ => None,
        }
    }
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_long())
    }
}

/// Normalization format for time units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizeFormat {
    Long,
    Short,
}

/// Normalizes a time unit string to a specified format.
///
/// Converts time unit strings between long format (milliseconds, seconds, minutes, hours)
/// and short format (ms, s, m, h).
///
/// # Arguments
///
/// * `unit` - A string slice representing the time unit (long or short format).
/// * `to` - The target format to normalize to.
///
/// # Returns
///
/// `Some(String)` with the normalized time unit, or `None` if the input is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::time::{umt_normalize_time_unit, NormalizeFormat};
///
/// assert_eq!(umt_normalize_time_unit("ms", NormalizeFormat::Long), Some("milliseconds".to_string()));
/// assert_eq!(umt_normalize_time_unit("hours", NormalizeFormat::Short), Some("h".to_string()));
/// ```
pub fn umt_normalize_time_unit(unit: &str, to: NormalizeFormat) -> Option<String> {
    let time_unit = TimeUnit::from_str(unit)?;
    Some(match to {
        NormalizeFormat::Long => time_unit.to_long().to_string(),
        NormalizeFormat::Short => time_unit.to_short().to_string(),
    })
}
