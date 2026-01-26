use crate::internal::rng;
use std::collections::HashSet;

/// Randomly selects a specified number of elements from an array.
///
/// # Arguments
///
/// * `array` - Source array
/// * `count` - Number of elements to select
/// * `allow_duplicates` - Whether to allow duplicate selections (default: false)
///
/// # Returns
///
/// Vector of randomly selected elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_random_select;
///
/// let arr = vec![1, 2, 3, 4, 5];
/// let selected = umt_random_select(&arr, 2, false);
/// assert_eq!(selected.len(), 2);
/// ```
pub fn umt_random_select<T: Clone>(array: &[T], count: usize, allow_duplicates: bool) -> Vec<T> {
    if array.is_empty() || count == 0 {
        return vec![];
    }

    let mut result = Vec::with_capacity(count);

    if allow_duplicates {
        for _ in 0..count {
            let random_index = rng::random_range_usize(0, array.len() - 1);
            result.push(array[random_index].clone());
        }
    } else {
        let mut used_indices = HashSet::new();
        let max_selections = count.min(array.len());

        while result.len() < max_selections {
            let random_index = rng::random_range_usize(0, array.len() - 1);
            if used_indices.insert(random_index) {
                result.push(array[random_index].clone());
            }
        }
    }

    result
}

/// Randomly selects a single element from an array.
///
/// # Arguments
///
/// * `array` - Source array
///
/// # Returns
///
/// A randomly selected element, or None if the array is empty
pub fn umt_random_select_one<T: Clone>(array: &[T]) -> Option<T> {
    if array.is_empty() {
        return None;
    }

    let random_index = rng::random_range_usize(0, array.len() - 1);
    Some(array[random_index].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_select_count() {
        let arr = vec![1, 2, 3, 4, 5];
        let selected = umt_random_select(&arr, 3, false);
        assert_eq!(selected.len(), 3);
    }

    #[test]
    fn test_random_select_no_duplicates() {
        let arr = vec![1, 2, 3, 4, 5];
        let selected = umt_random_select(&arr, 5, false);

        // All elements should be unique
        let unique: HashSet<_> = selected.iter().collect();
        assert_eq!(unique.len(), 5);
    }

    #[test]
    fn test_random_select_elements_from_source() {
        let arr = vec![1, 2, 3, 4, 5];
        let selected = umt_random_select(&arr, 3, false);

        for item in &selected {
            assert!(arr.contains(item));
        }
    }

    #[test]
    fn test_random_select_more_than_array_no_duplicates() {
        let arr = vec![1, 2, 3];
        let selected = umt_random_select(&arr, 10, false);

        // Without duplicates, can only select up to array length
        assert_eq!(selected.len(), 3);
    }

    #[test]
    fn test_random_select_with_duplicates() {
        let arr = vec![1, 2, 3];
        let selected = umt_random_select(&arr, 10, true);

        // With duplicates, can select the requested count
        assert_eq!(selected.len(), 10);
    }

    #[test]
    fn test_random_select_empty_array() {
        let arr: Vec<i32> = vec![];
        let selected = umt_random_select(&arr, 5, false);
        assert!(selected.is_empty());
    }

    #[test]
    fn test_random_select_zero_count() {
        let arr = vec![1, 2, 3, 4, 5];
        let selected = umt_random_select(&arr, 0, false);
        assert!(selected.is_empty());
    }

    #[test]
    fn test_random_select_one() {
        let arr = vec![1, 2, 3, 4, 5];
        let selected = umt_random_select_one(&arr);

        assert!(selected.is_some());
        assert!(arr.contains(&selected.unwrap()));
    }

    #[test]
    fn test_random_select_one_empty() {
        let arr: Vec<i32> = vec![];
        let selected = umt_random_select_one(&arr);
        assert!(selected.is_none());
    }

    #[test]
    fn test_random_select_strings() {
        let arr = vec!["a", "b", "c", "d", "e"];
        let selected = umt_random_select(&arr, 3, false);

        assert_eq!(selected.len(), 3);
        for item in &selected {
            assert!(arr.contains(item));
        }
    }
}
