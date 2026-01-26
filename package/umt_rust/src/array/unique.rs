use std::collections::HashSet;
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
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for item in array {
        if seen.insert(item.clone()) {
            result.push(item.clone());
        }
    }

    result
}

/// Removes duplicate values from a vector of f64, handling NaN values.
/// NaN values are considered equal to each other.
///
/// # Arguments
///
/// * `array` - The array to process
///
/// # Returns
///
/// A new vector with unique values
pub fn umt_unique_f64(array: &[f64]) -> Vec<f64> {
    let mut result = Vec::new();
    let mut has_nan = false;

    for &item in array {
        if item.is_nan() {
            if !has_nan {
                result.push(item);
                has_nan = true;
            }
        } else if !result.contains(&item) {
            result.push(item);
        }
    }

    result
}
