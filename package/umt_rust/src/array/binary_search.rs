/// Binary search implementation.
/// Searches for a target value in a sorted array.
///
/// # Arguments
///
/// * `array` - A sorted array of numbers
/// * `target` - The value to search for
///
/// # Returns
///
/// The index of the target value in the array, or None if not found
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_binary_search;
///
/// let arr = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
/// assert_eq!(umt_binary_search(&arr, 7), Some(3));
/// assert_eq!(umt_binary_search(&arr, 2), None);
/// ```
pub fn umt_binary_search<T: PartialOrd>(array: &[T], target: T) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if array[mid] == target {
            return Some(mid);
        }

        if array[mid] < target {
            left = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            right = mid - 1;
        }
    }

    None
}

/// Binary search that returns the index as i32.
/// Returns -1 if not found (matching TypeScript behavior).
///
/// # Arguments
///
/// * `array` - A sorted array of numbers
/// * `target` - The value to search for
///
/// # Returns
///
/// The index of the target value, or -1 if not found
#[inline]
pub fn umt_binary_search_i32<T: PartialOrd>(array: &[T], target: T) -> i32 {
    match umt_binary_search(array, target) {
        Some(idx) => idx as i32,
        None => -1,
    }
}
