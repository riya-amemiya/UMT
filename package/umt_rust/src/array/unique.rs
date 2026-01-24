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
        } else if !result.iter().any(|&x| x == item) {
            result.push(item);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_integers() {
        assert_eq!(umt_unique(&[1, 2, 2, 3, 3, 3]), vec![1, 2, 3]);
        assert_eq!(umt_unique(&[1, 1, 1, 1]), vec![1]);
        assert_eq!(umt_unique(&[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_unique_empty() {
        assert_eq!(umt_unique::<i32>(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_unique_single() {
        assert_eq!(umt_unique(&[42]), vec![42]);
    }

    #[test]
    fn test_unique_strings() {
        assert_eq!(
            umt_unique(&["a", "b", "a", "c", "b"]),
            vec!["a", "b", "c"]
        );
    }

    #[test]
    fn test_unique_preserves_order() {
        assert_eq!(umt_unique(&[3, 1, 2, 1, 3, 2]), vec![3, 1, 2]);
    }

    #[test]
    fn test_unique_f64() {
        assert_eq!(umt_unique_f64(&[1.0, 2.0, 2.0, 3.0]), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_unique_f64_with_nan() {
        let arr = vec![1.0, f64::NAN, 2.0, f64::NAN, 3.0];
        let result = umt_unique_f64(&arr);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], 1.0);
        assert!(result[1].is_nan());
        assert_eq!(result[2], 2.0);
        assert_eq!(result[3], 3.0);
    }
}
