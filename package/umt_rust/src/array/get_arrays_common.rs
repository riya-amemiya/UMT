use super::ordered_unique::ordered_unique_by;
use super::unique::same_value_zero_bits;
use std::collections::HashSet;
use std::hash::Hash;

/// Extract common elements from multiple arrays.
/// Returns unique elements that exist in all input arrays.
/// A single input array is returned as is.
///
/// # Arguments
///
/// * `arrays` - Vector of arrays to compare
///
/// # Returns
///
/// Vector containing common elements (deduplicated)
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_get_arrays_common;
///
/// assert_eq!(
///     umt_get_arrays_common(&[&[1, 2, 3][..], &[2, 3, 4][..], &[2, 5, 3][..]]),
///     vec![2, 3]
/// );
/// ```
pub fn umt_get_arrays_common<T: Clone + Hash + Eq>(arrays: &[&[T]]) -> Vec<T> {
    if arrays.is_empty() {
        return vec![];
    }

    if arrays.len() == 1 {
        return arrays[0].to_vec();
    }

    let other_sets: Vec<HashSet<&T>> = arrays[1..].iter().map(|arr| arr.iter().collect()).collect();

    ordered_unique_by(
        arrays[0]
            .iter()
            .filter(|item| other_sets.iter().all(|set| set.contains(item))),
        |item| item,
    )
}

/// Extract common elements from exactly two arrays.
/// Convenience function for comparing two arrays.
///
/// # Arguments
///
/// * `array1` - First array
/// * `array2` - Second array
///
/// # Returns
///
/// Vector containing common elements
#[inline]
pub fn umt_get_arrays_common_two<T: Clone + Hash + Eq>(array1: &[T], array2: &[T]) -> Vec<T> {
    umt_get_arrays_common(&[array1, array2])
}

/// Extract common elements from multiple arrays of f64, handling NaN values.
/// NaN is considered equal to NaN, and 0.0 is considered equal to -0.0.
/// A single input array is returned as is.
///
/// # Arguments
///
/// * `arrays` - Vector of arrays to compare
///
/// # Returns
///
/// Vector containing common elements
pub fn umt_get_arrays_common_f64(arrays: &[&[f64]]) -> Vec<f64> {
    if arrays.is_empty() {
        return vec![];
    }

    if arrays.len() == 1 {
        return arrays[0].to_vec();
    }

    let other_sets: Vec<HashSet<u64>> = arrays[1..]
        .iter()
        .map(|arr| arr.iter().map(|&x| same_value_zero_bits(x)).collect())
        .collect();

    ordered_unique_by(
        arrays[0].iter().filter(|&&item| {
            let key = same_value_zero_bits(item);
            other_sets.iter().all(|set| set.contains(&key))
        }),
        |&item| same_value_zero_bits(item),
    )
}
