use super::ordered_unique::ordered_unique_by;
use std::hash::Hash;

/// Removes duplicate values from an array.
///
/// # Arguments
///
/// * `array` - The array to process
///
/// # Returns
///
/// A new vector with unique values
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_unique;
///
/// assert_eq!(umt_unique(&[1, 2, 2, 3, 3, 3]), vec![1, 2, 3]);
/// ```
pub fn umt_unique<T: Clone + Hash + Eq>(array: &[T]) -> Vec<T> {
    ordered_unique_by(array, |item| item)
}

pub(super) fn same_value_zero_bits(value: f64) -> u64 {
    if value.is_nan() {
        f64::NAN.to_bits()
    } else if value == 0.0 {
        0
    } else {
        value.to_bits()
    }
}

/// Removes duplicate values from a vector of f64, handling NaN values.
/// NaN values are considered equal to each other, and 0.0 is considered equal to -0.0.
///
/// Uses a HashSet of bit representations for O(N) deduplication instead of
/// O(N²) linear search via `Vec::contains`.
///
/// # Arguments
///
/// * `array` - The array to process
///
/// # Returns
///
/// A new vector with unique values
pub fn umt_unique_f64(array: &[f64]) -> Vec<f64> {
    ordered_unique_by(array, |&item| same_value_zero_bits(item))
}
