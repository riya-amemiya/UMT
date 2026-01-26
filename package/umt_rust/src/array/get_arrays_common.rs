use std::collections::HashSet;
use std::hash::Hash;

/// Extract common elements from multiple arrays.
/// Returns unique elements that exist in all input arrays.
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
        // Return unique elements from the single array
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        for item in arrays[0] {
            if seen.insert(item.clone()) {
                result.push(item.clone());
            }
        }
        return result;
    }

    // Create sets for all arrays except the first
    let other_sets: Vec<HashSet<T>> = arrays[1..]
        .iter()
        .map(|arr| arr.iter().cloned().collect())
        .collect();

    // Find elements in first array that exist in all other arrays
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for item in arrays[0] {
        if !seen.contains(item) && other_sets.iter().all(|set| set.contains(item)) {
            seen.insert(item.clone());
            result.push(item.clone());
        }
    }

    result
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
/// NaN is considered equal to NaN.
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
        let mut result = Vec::new();
        let mut has_nan = false;
        for &item in arrays[0] {
            if item.is_nan() {
                if !has_nan {
                    result.push(item);
                    has_nan = true;
                }
            } else if !result.contains(&item) {
                result.push(item);
            }
        }
        return result;
    }

    // Check if all arrays contain NaN
    let all_have_nan = arrays.iter().all(|arr| arr.iter().any(|x| x.is_nan()));

    let mut result = Vec::new();
    let mut has_nan_in_result = false;

    for &item in arrays[0] {
        if item.is_nan() {
            if all_have_nan && !has_nan_in_result {
                result.push(item);
                has_nan_in_result = true;
            }
        } else {
            let in_all = arrays[1..].iter().all(|arr| arr.contains(&item));
            if in_all && !result.contains(&item) {
                result.push(item);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_arrays_common_basic() {
        assert_eq!(
            umt_get_arrays_common(&[&[1, 2, 3][..], &[2, 3, 4][..], &[2, 5, 3][..]]),
            vec![2, 3]
        );
    }

    #[test]
    fn test_get_arrays_common_two_arrays() {
        assert_eq!(
            umt_get_arrays_common(&[&[1, 2, 3][..], &[2, 3, 4][..]]),
            vec![2, 3]
        );
    }

    #[test]
    fn test_get_arrays_common_no_common() {
        assert_eq!(
            umt_get_arrays_common(&[&[1, 2, 3][..], &[4, 5, 6][..]]),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_get_arrays_common_single_array() {
        assert_eq!(umt_get_arrays_common(&[&[1, 2, 3][..]]), vec![1, 2, 3]);
    }

    #[test]
    fn test_get_arrays_common_empty() {
        let empty: &[&[i32]] = &[];
        assert_eq!(umt_get_arrays_common(empty), Vec::<i32>::new());
    }

    #[test]
    fn test_get_arrays_common_with_duplicates() {
        assert_eq!(
            umt_get_arrays_common(&[&[1, 1, 2, 2][..], &[2, 2, 3, 3][..]]),
            vec![2]
        );
    }

    #[test]
    fn test_get_arrays_common_strings() {
        assert_eq!(
            umt_get_arrays_common(&[&["a", "b", "c"][..], &["b", "c", "d"][..]]),
            vec!["b", "c"]
        );
    }

    #[test]
    fn test_get_arrays_common_two_convenience() {
        assert_eq!(
            umt_get_arrays_common_two(&[1, 2, 3], &[2, 3, 4]),
            vec![2, 3]
        );
    }

    #[test]
    fn test_get_arrays_common_f64() {
        assert_eq!(
            umt_get_arrays_common_f64(&[&[1.0, 2.0, 3.0][..], &[2.0, 3.0, 4.0][..]]),
            vec![2.0, 3.0]
        );
    }

    #[test]
    fn test_get_arrays_common_f64_with_nan() {
        let arr1 = [1.0, f64::NAN, 2.0];
        let arr2 = [f64::NAN, 2.0, 3.0];
        let result = umt_get_arrays_common_f64(&[&arr1[..], &arr2[..]]);

        assert_eq!(result.len(), 2);
        assert!(result[0].is_nan());
        assert_eq!(result[1], 2.0);
    }
}
