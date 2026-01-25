use std::collections::HashSet;
use std::hash::Hash;

/// Extract elements that are not common between arrays.
/// Returns elements that appear only once across all arrays.
///
/// # Arguments
///
/// * `arrays` - Vector of arrays to compare
///
/// # Returns
///
/// Vector containing elements that appear only once
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_get_arrays_diff;
///
/// assert_eq!(umt_get_arrays_diff(&[&[1, 2, 3][..], &[2, 3, 4][..]]), vec![1, 4]);
/// ```
pub fn umt_get_arrays_diff<T: Clone + Hash + Eq>(arrays: &[&[T]]) -> Vec<T> {
    if arrays.is_empty() {
        return vec![];
    }

    let mut all_values = HashSet::new();
    let mut duplicates = HashSet::new();

    // First array - add all to all_values
    if let Some(first) = arrays.first() {
        for item in *first {
            all_values.insert(item.clone());
        }
    }

    // Remaining arrays - check for duplicates
    for array in arrays.iter().skip(1) {
        for item in *array {
            if all_values.contains(item) {
                duplicates.insert(item.clone());
            } else {
                all_values.insert(item.clone());
            }
        }
    }

    // Return elements that are not duplicates, preserving order
    let mut result = Vec::new();
    let mut seen = HashSet::new();

    for array in arrays {
        for item in *array {
            if !duplicates.contains(item) && seen.insert(item.clone()) {
                result.push(item.clone());
            }
        }
    }

    result
}

/// Extract elements that differ between exactly two arrays.
/// Convenience function for comparing two arrays.
///
/// # Arguments
///
/// * `array1` - First array
/// * `array2` - Second array
///
/// # Returns
///
/// Vector containing elements unique to each array
#[inline]
pub fn umt_get_arrays_diff_two<T: Clone + Hash + Eq>(array1: &[T], array2: &[T]) -> Vec<T> {
    umt_get_arrays_diff(&[array1, array2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_arrays_diff_basic() {
        assert_eq!(
            umt_get_arrays_diff(&[&[1, 2, 3][..], &[2, 3, 4][..]]),
            vec![1, 4]
        );
    }

    #[test]
    fn test_get_arrays_diff_no_diff() {
        assert_eq!(
            umt_get_arrays_diff(&[&[1, 2, 3][..], &[1, 2, 3][..]]),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_get_arrays_diff_all_diff() {
        assert_eq!(
            umt_get_arrays_diff(&[&[1, 2][..], &[3, 4][..]]),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_get_arrays_diff_single_array() {
        assert_eq!(umt_get_arrays_diff(&[&[1, 2, 3][..]]), vec![1, 2, 3]);
    }

    #[test]
    fn test_get_arrays_diff_empty() {
        let empty: &[&[i32]] = &[];
        assert_eq!(umt_get_arrays_diff(empty), Vec::<i32>::new());
    }

    #[test]
    fn test_get_arrays_diff_multiple_arrays() {
        assert_eq!(
            umt_get_arrays_diff(&[&[1, 2, 3][..], &[2, 3, 4][..], &[3, 4, 5][..]]),
            vec![1, 5]
        );
    }

    #[test]
    fn test_get_arrays_diff_strings() {
        assert_eq!(
            umt_get_arrays_diff(&[&["a", "b", "c"][..], &["b", "c", "d"][..]]),
            vec!["a", "d"]
        );
    }

    #[test]
    fn test_get_arrays_diff_two_convenience() {
        assert_eq!(umt_get_arrays_diff_two(&[1, 2, 3], &[2, 3, 4]), vec![1, 4]);
    }

    #[test]
    fn test_get_arrays_diff_with_duplicates_in_same_array() {
        // [1, 1, 2]: all_values = {1, 2}
        // [2, 3, 3]: 2 is duplicate (in all_values), 3 is added, second 3 is also duplicate
        // duplicates = {2, 3}
        // Result = all_values - duplicates = {1}
        let result = umt_get_arrays_diff(&[&[1, 1, 2][..], &[2, 3, 3][..]]);
        assert_eq!(result, vec![1]);
    }
}
