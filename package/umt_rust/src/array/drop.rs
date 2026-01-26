/// Direction for dropping elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropDirection {
    Left,
    Right,
    Between,
}

/// Returns a new vector with n elements removed from the specified direction.
///
/// # Arguments
///
/// * `array` - The target array
/// * `n` - The number of elements to remove
/// * `direction` - The direction to remove elements from (Left, Right, or Between)
///
/// # Returns
///
/// A new vector with n elements removed
///
/// # Examples
///
/// ```
/// use umt_rust::array::{umt_drop, DropDirection};
///
/// assert_eq!(umt_drop(&[1, 2, 3, 4, 5], 2, DropDirection::Left), vec![3, 4, 5]);
/// assert_eq!(umt_drop(&[1, 2, 3, 4, 5], 2, DropDirection::Right), vec![1, 2, 3]);
/// assert_eq!(umt_drop(&[1, 2, 3, 4, 5], 1, DropDirection::Between), vec![1, 2, 4, 5]);
/// ```
pub fn umt_drop<T: Clone>(array: &[T], n: usize, direction: DropDirection) -> Vec<T> {
    if array.is_empty() {
        return vec![];
    }

    match direction {
        DropDirection::Left => {
            if n >= array.len() {
                vec![]
            } else {
                array[n..].to_vec()
            }
        }
        DropDirection::Right => {
            if n >= array.len() {
                vec![]
            } else {
                array[..array.len() - n].to_vec()
            }
        }
        DropDirection::Between => {
            let len = array.len();
            let mid = len / 2;
            let start = mid.saturating_sub(n / 2);
            let end = (mid + n.div_ceil(2)).min(len);

            let mut result = array[..start].to_vec();
            result.extend_from_slice(&array[end..]);
            result
        }
    }
}

/// Returns a new vector with n elements removed from the left (start).
/// Convenience function equivalent to `umt_drop(array, n, DropDirection::Left)`.
///
/// # Arguments
///
/// * `array` - The target array
/// * `n` - The number of elements to remove (default: 1)
///
/// # Returns
///
/// A new vector with n elements removed from the start
#[inline]
pub fn umt_drop_left<T: Clone>(array: &[T], n: usize) -> Vec<T> {
    umt_drop(array, n, DropDirection::Left)
}

/// Returns a new vector with n elements removed from the right (end).
/// Convenience function equivalent to `umt_drop(array, n, DropDirection::Right)`.
///
/// # Arguments
///
/// * `array` - The target array
/// * `n` - The number of elements to remove
///
/// # Returns
///
/// A new vector with n elements removed from the end
#[inline]
pub fn umt_drop_right<T: Clone>(array: &[T], n: usize) -> Vec<T> {
    umt_drop(array, n, DropDirection::Right)
}
