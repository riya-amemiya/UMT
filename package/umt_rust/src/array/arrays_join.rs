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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrays_join_basic() {
        assert_eq!(
            umt_arrays_join(&[&[1, 2, 3][..], &[4, 5, 6][..]]),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_arrays_join_multiple() {
        assert_eq!(
            umt_arrays_join(&[&[1, 2, 3][..], &[4, 5, 6][..], &[7, 8, 9][..]]),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn test_arrays_join_with_duplicates() {
        assert_eq!(
            umt_arrays_join(&[&[1, 2, 3][..], &[2, 3, 4][..], &[3, 4, 5][..]]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_arrays_join_empty() {
        let empty: &[i32] = &[];
        assert_eq!(umt_arrays_join(&[empty, &[1, 2, 3][..]]), vec![1, 2, 3]);
        assert_eq!(umt_arrays_join(&[&[1, 2, 3][..], empty]), vec![1, 2, 3]);
    }

    #[test]
    fn test_arrays_join_strings() {
        assert_eq!(
            umt_arrays_join(&[&["a", "b", "c"][..], &["b", "c", "d"][..]]),
            vec!["a", "b", "c", "d"]
        );
    }

    #[test]
    fn test_arrays_join_two_convenience() {
        assert_eq!(
            umt_arrays_join_two(&[1, 2, 3], &[2, 3, 4]),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_arrays_join_preserves_order() {
        assert_eq!(
            umt_arrays_join(&[&[3, 1, 2][..], &[2, 4, 1][..]]),
            vec![3, 1, 2, 4]
        );
    }

    #[test]
    fn test_arrays_join_single_array() {
        assert_eq!(umt_arrays_join(&[&[1, 2, 3][..]]), vec![1, 2, 3]);
    }

    #[test]
    fn test_arrays_join_single_with_duplicates() {
        assert_eq!(umt_arrays_join(&[&[1, 1, 2, 2, 3, 3][..]]), vec![1, 2, 3]);
    }

    #[test]
    fn test_arrays_join_f64_with_nan() {
        let arr1 = [1.0, f64::NAN, 2.0];
        let arr2 = [f64::NAN, 3.0];
        let result = umt_arrays_join_f64(&[&arr1[..], &arr2[..]]);

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], 1.0);
        assert!(result[1].is_nan());
        assert_eq!(result[2], 2.0);
        assert_eq!(result[3], 3.0);
    }
}
