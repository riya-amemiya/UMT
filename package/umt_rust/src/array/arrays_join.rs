use std::collections::HashSet;
use std::hash::Hash;

/// Join arrays without duplicates.
/// Elements are deduplicated while preserving the order of first occurrence.
///
/// # Arguments
///
/// * `arrays` - Vector of arrays to join
///
/// # Returns
///
/// Vector with unique elements from all input arrays
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_arrays_join;
///
/// assert_eq!(umt_arrays_join(&[&[1, 2, 3][..], &[2, 3, 4][..]]), vec![1, 2, 3, 4]);
/// ```
pub fn umt_arrays_join<T: Clone + Hash + Eq>(arrays: &[&[T]]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for array in arrays {
        for item in *array {
            if seen.insert(item.clone()) {
                result.push(item.clone());
            }
        }
    }

    result
}

/// Join two arrays without duplicates.
/// Convenience function for joining exactly two arrays.
///
/// # Arguments
///
/// * `array1` - First array
/// * `array2` - Second array
///
/// # Returns
///
/// Vector with unique elements from both arrays
pub fn umt_arrays_join_two<T: Clone + Hash + Eq>(array1: &[T], array2: &[T]) -> Vec<T> {
    umt_arrays_join(&[array1, array2])
}

/// Join arrays of f64 without duplicates, handling NaN values.
/// NaN values are considered equal to each other.
///
/// # Arguments
///
/// * `arrays` - Vector of arrays to join
///
/// # Returns
///
/// Vector with unique elements from all input arrays
pub fn umt_arrays_join_f64(arrays: &[&[f64]]) -> Vec<f64> {
    let mut result = Vec::new();
    let mut has_nan = false;

    for array in arrays {
        for &item in *array {
            if item.is_nan() {
                if !has_nan {
                    result.push(item);
                    has_nan = true;
                }
            } else if !result.contains(&item) {
                result.push(item);
            }
        }
    }

    result
}
