const DURATION_UNITS: [(&str, u64, Option<u64>); 5] = [
    ("d", 86_400_000, None),
    ("h", 3_600_000, Some(24)),
    ("m", 60_000, Some(60)),
    ("s", 1_000, Some(60)),
    ("ms", 1, Some(1_000)),
];

/// Formats a millisecond duration as a compact human-readable string.
///
/// Units are days (`d`), hours (`h`), minutes (`m`), seconds (`s`) and
/// milliseconds (`ms`), separated by a space. Zero units are omitted,
/// fractional milliseconds are truncated, and a zero duration returns `"0s"`.
/// Negative durations keep a leading `"-"`.
///
/// # Arguments
///
/// * `milliseconds` - Duration in milliseconds
///
/// # Returns
///
/// The formatted duration.
///
/// # Examples
///
/// ```
/// use umt_rust::time::umt_format_duration;
///
/// assert_eq!(umt_format_duration(5_400_000.0), "1h 30m");
/// assert_eq!(umt_format_duration(45_000.0), "45s");
/// assert_eq!(umt_format_duration(183_600_000.0), "2d 3h");
/// assert_eq!(umt_format_duration(1_500.0), "1s 500ms");
/// assert_eq!(umt_format_duration(-90_000.0), "-1m 30s");
/// assert_eq!(umt_format_duration(0.0), "0s");
/// ```
pub fn umt_format_duration(milliseconds: f64) -> String {
    let total = milliseconds.abs().trunc() as u64;
    let parts: Vec<String> = DURATION_UNITS
        .iter()
        .map(|&(label, size, modulus)| {
            let value = total / size;
            (label, modulus.map_or(value, |limit| value % limit))
        })
        .filter(|&(_, value)| value > 0)
        .map(|(label, value)| format!("{}{}", value, label))
        .collect();
    if parts.is_empty() {
        return "0s".to_string();
    }
    let sign = if milliseconds < 0.0 { "-" } else { "" };
    format!("{}{}", sign, parts.join(" "))
}
