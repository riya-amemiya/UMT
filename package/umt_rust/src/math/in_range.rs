/// Checks if a number is within a given range.
///
/// This function checks if the value is between the start and end values.
/// The range is [min(start, end), max(start, end)).
///
/// # Arguments
///
/// * `value` - The number to check.
/// * `start` - The start of the range.
/// * `end` - The end of the range.
///
/// # Returns
///
/// True if the value is within the range, false otherwise.
#[inline]
pub fn umt_in_range(value: f64, start: f64, end: f64) -> bool {
    if value.is_nan() || start.is_nan() || end.is_nan() {
        return false;
    }
    let lower = start.min(end);
    let upper = start.max(end);
    value >= lower && value < upper
}
