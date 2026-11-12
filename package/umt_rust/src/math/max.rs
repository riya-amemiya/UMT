/// Returns the maximum value from a slice of numbers.
///
/// This function takes a slice of f64 numbers and returns the maximum value.
/// Duplicates are automatically removed during processing.
///
/// # Arguments
///
/// * `numbers` - A slice of f64 numbers.
///
/// # Returns
///
/// The maximum value from the input slice, f64::NEG_INFINITY if empty, or f64::NAN if any value is NaN.
#[inline]
pub fn umt_max(numbers: &[f64]) -> f64 {
    numbers.iter().fold(f64::NEG_INFINITY, |acc, &x| {
        if x.is_nan() || acc.is_nan() {
            f64::NAN
        } else if x > acc {
            x
        } else {
            acc
        }
    })
}
