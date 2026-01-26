use rand::Rng;
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

    let mut rng = rand::rng();
    let mut result = Vec::with_capacity(count);

    if allow_duplicates {
        for _ in 0..count {
            let random_index = rng.random_range(0..array.len());
            result.push(array[random_index].clone());
        }
    } else {
        let mut used_indices = HashSet::new();
        let max_selections = count.min(array.len());

        while result.len() < max_selections {
            let random_index = rng.random_range(0..array.len());
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

    let mut rng = rand::rng();
    let random_index = rng.random_range(0..array.len());
    Some(array[random_index].clone())
}
