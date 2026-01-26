/// Returns the minimum value from a slice of numbers.
///
/// This function takes a slice of f64 numbers and returns the minimum value.
/// Duplicates are automatically removed during processing.
///
/// # Arguments
///
/// * `numbers` - A slice of f64 numbers.
///
/// # Returns
///
/// The minimum value from the input slice, or f64::INFINITY if empty.
#[inline]
pub fn umt_min(numbers: &[f64]) -> f64 {
    numbers
        .iter()
        .fold(f64::INFINITY, |acc, &x| if x < acc { x } else { acc })
}
