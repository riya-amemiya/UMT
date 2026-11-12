const BYTE_UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];

/// Formats a byte count as a human-readable string using base 1024.
///
/// The unit is the largest of B, KB, MB, GB, TB and PB that does not exceed
/// the absolute value (values of a PB or more stay in PB). The value is
/// rounded to `decimals` places with JavaScript `Math.round` semantics and
/// printed without trailing zeros.
///
/// # Arguments
///
/// * `bytes` - The number of bytes (may be negative or fractional)
/// * `decimals` - The number of decimal places
///
/// # Returns
///
/// * The formatted size, such as `"1.5 KB"`
///
/// # Examples
///
/// ```
/// use umt_rust::number::umt_format_bytes;
///
/// assert_eq!(umt_format_bytes(0.0, 2), "0 B");
/// assert_eq!(umt_format_bytes(1536.0, 2), "1.5 KB");
/// assert_eq!(umt_format_bytes(1_234_567.0, 2), "1.18 MB");
/// assert_eq!(umt_format_bytes(1_234_567.0, 0), "1 MB");
/// assert_eq!(umt_format_bytes(-2048.0, 2), "-2 KB");
/// ```
pub fn umt_format_bytes(bytes: f64, decimals: i32) -> String {
    let absolute = bytes.abs();
    let index = (0..BYTE_UNITS.len())
        .rev()
        .find(|&unit_index| absolute >= 1024_f64.powi(unit_index as i32))
        .unwrap_or(0);
    let factor = 10_f64.powi(decimals);
    let value = ((bytes / 1024_f64.powi(index as i32)) * factor + 0.5).floor() / factor + 0.0;
    format!("{} {}", value, BYTE_UNITS[index])
}

/// Formats a byte count as a human-readable string with 2 decimal places.
///
/// # Arguments
///
/// * `bytes` - The number of bytes (may be negative or fractional)
///
/// # Returns
///
/// * The formatted size, such as `"1.5 KB"`
///
/// # Examples
///
/// ```
/// use umt_rust::number::umt_format_bytes_default;
///
/// assert_eq!(umt_format_bytes_default(1024.0), "1 KB");
/// ```
pub fn umt_format_bytes_default(bytes: f64) -> String {
    umt_format_bytes(bytes, 2)
}
