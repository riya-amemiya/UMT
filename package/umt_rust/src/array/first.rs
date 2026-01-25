/// Returns the first element of an array, or None if the array is empty.
///
/// # Arguments
///
/// * `array` - The input array
///
/// # Returns
///
/// The first element of the array, or None if the array is empty.
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_first;
///
/// assert_eq!(umt_first(&[1, 2, 3]), Some(&1));
/// assert_eq!(umt_first::<i32>(&[]), None);
/// ```
#[inline]
pub fn umt_first<T>(array: &[T]) -> Option<&T> {
    array.first()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_returns_first_element() {
        assert_eq!(umt_first(&[1, 2, 3]), Some(&1));
        assert_eq!(umt_first(&["a", "b", "c"]), Some(&"a"));
    }

    #[test]
    fn test_first_empty_array() {
        assert_eq!(umt_first::<i32>(&[]), None);
        assert_eq!(umt_first::<String>(&[]), None);
    }

    #[test]
    fn test_first_single_element() {
        assert_eq!(umt_first(&[42]), Some(&42));
    }

    #[test]
    fn test_first_with_strings() {
        let arr = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(umt_first(&arr), Some(&"hello".to_string()));
    }
}
