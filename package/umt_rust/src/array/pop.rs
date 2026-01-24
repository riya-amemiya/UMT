/// Returns the last element of an array, or None if the array is empty.
/// This function does not modify the original array.
///
/// # Arguments
///
/// * `array` - The input array
///
/// # Returns
///
/// The last element of the array, or None if the array is empty.
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_pop;
///
/// assert_eq!(umt_pop(&[1, 2, 3]), Some(&3));
/// assert_eq!(umt_pop::<i32>(&[]), None);
/// ```
#[inline]
pub fn umt_pop<T>(array: &[T]) -> Option<&T> {
    array.last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_returns_last_element() {
        assert_eq!(umt_pop(&[1, 2, 3]), Some(&3));
        assert_eq!(umt_pop(&["a", "b", "c"]), Some(&"c"));
    }

    #[test]
    fn test_pop_empty_array() {
        assert_eq!(umt_pop::<i32>(&[]), None);
        assert_eq!(umt_pop::<String>(&[]), None);
    }

    #[test]
    fn test_pop_single_element() {
        assert_eq!(umt_pop(&[42]), Some(&42));
    }

    #[test]
    fn test_pop_with_strings() {
        let arr = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(umt_pop(&arr), Some(&"world".to_string()));
    }

    #[test]
    fn test_pop_does_not_modify_array() {
        let arr = vec![1, 2, 3];
        let _ = umt_pop(&arr);
        assert_eq!(arr, vec![1, 2, 3]);
    }
}
