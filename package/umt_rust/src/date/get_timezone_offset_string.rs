//! Timezone offset string utility.
//!
//! This module provides functions to get timezone offset strings.

use crate::internal::datetime;

/// Get timezone offset string in format "+HH:mm" or "-HH:mm".
///
/// # Arguments
///
/// * `offset_minutes` - The timezone offset in minutes from UTC
///
/// # Returns
///
/// The timezone offset string (e.g. "+09:00" for JST)
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_get_timezone_offset_string;
///
/// assert_eq!(umt_get_timezone_offset_string(540), "+09:00");   // JST
/// assert_eq!(umt_get_timezone_offset_string(0), "+00:00");     // UTC
/// assert_eq!(umt_get_timezone_offset_string(-300), "-05:00");  // EST
/// ```
#[inline]
pub fn umt_get_timezone_offset_string(offset_minutes: i32) -> String {
    let sign = if offset_minutes >= 0 { "+" } else { "-" };
    let abs_minutes = offset_minutes.abs();
    let hours = abs_minutes / 60;
    let minutes = abs_minutes % 60;

    format!("{}{:02}:{:02}", sign, hours, minutes)
}

/// Get timezone offset string without colon in format "+HHmm" or "-HHmm".
///
/// # Arguments
///
/// * `offset_minutes` - The timezone offset in minutes from UTC
///
/// # Returns
///
/// The timezone offset string without colon (e.g. "+0900" for JST)
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_get_timezone_offset_string_compact;
///
/// assert_eq!(umt_get_timezone_offset_string_compact(540), "+0900");   // JST
/// assert_eq!(umt_get_timezone_offset_string_compact(0), "+0000");     // UTC
/// assert_eq!(umt_get_timezone_offset_string_compact(-300), "-0500");  // EST
/// ```
#[inline]
pub fn umt_get_timezone_offset_string_compact(offset_minutes: i32) -> String {
    let sign = if offset_minutes >= 0 { "+" } else { "-" };
    let abs_minutes = offset_minutes.abs();
    let hours = abs_minutes / 60;
    let minutes = abs_minutes % 60;

    format!("{}{:02}{:02}", sign, hours, minutes)
}

/// Get the local timezone offset string.
///
/// # Returns
///
/// The local timezone offset string (e.g. "+09:00" for JST)
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_get_local_timezone_offset_string;
///
/// let offset = umt_get_local_timezone_offset_string();
/// // Will return something like "+09:00" depending on local timezone
/// ```
pub fn umt_get_local_timezone_offset_string() -> String {
    let offset_seconds = datetime::local_utc_offset_seconds();
    let offset_minutes = offset_seconds / 60;

    umt_get_timezone_offset_string(offset_minutes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_offset() {
        assert_eq!(umt_get_timezone_offset_string(540), "+09:00"); // JST
        assert_eq!(umt_get_timezone_offset_string(60), "+01:00");
        assert_eq!(umt_get_timezone_offset_string(330), "+05:30"); // IST
    }

    #[test]
    fn test_zero_offset() {
        assert_eq!(umt_get_timezone_offset_string(0), "+00:00");
    }

    #[test]
    fn test_negative_offset() {
        assert_eq!(umt_get_timezone_offset_string(-300), "-05:00"); // EST
        assert_eq!(umt_get_timezone_offset_string(-480), "-08:00"); // PST
        assert_eq!(umt_get_timezone_offset_string(-210), "-03:30"); // NST
    }

    #[test]
    fn test_compact_format() {
        assert_eq!(umt_get_timezone_offset_string_compact(540), "+0900");
        assert_eq!(umt_get_timezone_offset_string_compact(0), "+0000");
        assert_eq!(umt_get_timezone_offset_string_compact(-300), "-0500");
    }

    #[test]
    fn test_local_timezone_offset() {
        let offset = umt_get_local_timezone_offset_string();
        // Just verify it returns a valid format
        assert!(offset.starts_with('+') || offset.starts_with('-'));
        assert_eq!(offset.len(), 6);
        assert_eq!(&offset[3..4], ":");
    }
}
