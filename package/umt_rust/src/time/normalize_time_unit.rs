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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_unit_from_str() {
        assert_eq!(
            TimeUnit::from_str("milliseconds"),
            Some(TimeUnit::Milliseconds)
        );
        assert_eq!(TimeUnit::from_str("ms"), Some(TimeUnit::Milliseconds));
        assert_eq!(TimeUnit::from_str("seconds"), Some(TimeUnit::Seconds));
        assert_eq!(TimeUnit::from_str("s"), Some(TimeUnit::Seconds));
        assert_eq!(TimeUnit::from_str("minutes"), Some(TimeUnit::Minutes));
        assert_eq!(TimeUnit::from_str("m"), Some(TimeUnit::Minutes));
        assert_eq!(TimeUnit::from_str("hours"), Some(TimeUnit::Hours));
        assert_eq!(TimeUnit::from_str("h"), Some(TimeUnit::Hours));
        assert_eq!(TimeUnit::from_str("invalid"), None);
    }

    #[test]
    fn test_time_unit_to_long() {
        assert_eq!(TimeUnit::Milliseconds.to_long(), "milliseconds");
        assert_eq!(TimeUnit::Seconds.to_long(), "seconds");
        assert_eq!(TimeUnit::Minutes.to_long(), "minutes");
        assert_eq!(TimeUnit::Hours.to_long(), "hours");
    }

    #[test]
    fn test_time_unit_to_short() {
        assert_eq!(TimeUnit::Milliseconds.to_short(), "ms");
        assert_eq!(TimeUnit::Seconds.to_short(), "s");
        assert_eq!(TimeUnit::Minutes.to_short(), "m");
        assert_eq!(TimeUnit::Hours.to_short(), "h");
    }

    #[test]
    fn test_time_unit_to_milliseconds_rate() {
        assert_eq!(TimeUnit::Milliseconds.to_milliseconds_rate(), 1.0);
        assert_eq!(TimeUnit::Seconds.to_milliseconds_rate(), 1000.0);
        assert_eq!(TimeUnit::Minutes.to_milliseconds_rate(), 60_000.0);
        assert_eq!(TimeUnit::Hours.to_milliseconds_rate(), 3_600_000.0);
    }

    #[test]
    fn test_normalize_long_to_long() {
        assert_eq!(
            umt_normalize_time_unit("milliseconds", NormalizeFormat::Long),
            Some("milliseconds".to_string())
        );
        assert_eq!(
            umt_normalize_time_unit("seconds", NormalizeFormat::Long),
            Some("seconds".to_string())
        );
        assert_eq!(
            umt_normalize_time_unit("minutes", NormalizeFormat::Long),
            Some("minutes".to_string())
        );
        assert_eq!(
            umt_normalize_time_unit("hours", NormalizeFormat::Long),
            Some("hours".to_string())
        );
    }

    #[test]
    fn test_normalize_long_to_short() {
        assert_eq!(
            umt_normalize_time_unit("milliseconds", NormalizeFormat::Short),
            Some("ms".to_string())
        );
        assert_eq!(
            umt_normalize_time_unit("seconds", NormalizeFormat::Short),
            Some("s".to_string())
        );
        assert_eq!(
            umt_normalize_time_unit("minutes", NormalizeFormat::Short),
            Some("m".to_string())
        );
        assert_eq!(
            umt_normalize_time_unit("hours", NormalizeFormat::Short),
            Some("h".to_string())
        );
    }

    #[test]
    fn test_normalize_short_to_long() {
        assert_eq!(
            umt_normalize_time_unit("ms", NormalizeFormat::Long),
            Some("milliseconds".to_string())
        );
        assert_eq!(
            umt_normalize_time_unit("s", NormalizeFormat::Long),
            Some("seconds".to_string())
        );
        assert_eq!(
            umt_normalize_time_unit("m", NormalizeFormat::Long),
            Some("minutes".to_string())
        );
        assert_eq!(
            umt_normalize_time_unit("h", NormalizeFormat::Long),
            Some("hours".to_string())
        );
    }

    #[test]
    fn test_normalize_short_to_short() {
        assert_eq!(
            umt_normalize_time_unit("ms", NormalizeFormat::Short),
            Some("ms".to_string())
        );
        assert_eq!(
            umt_normalize_time_unit("s", NormalizeFormat::Short),
            Some("s".to_string())
        );
        assert_eq!(
            umt_normalize_time_unit("m", NormalizeFormat::Short),
            Some("m".to_string())
        );
        assert_eq!(
            umt_normalize_time_unit("h", NormalizeFormat::Short),
            Some("h".to_string())
        );
    }

    #[test]
    fn test_normalize_invalid_unit() {
        assert_eq!(
            umt_normalize_time_unit("invalid", NormalizeFormat::Long),
            None
        );
        assert_eq!(umt_normalize_time_unit("", NormalizeFormat::Short), None);
    }
}
