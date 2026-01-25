//! Empty object checking utility

use std::collections::HashMap;

/// Checks if a HashMap is not empty
///
/// # Arguments
/// * `object` - The HashMap to check
///
/// # Returns
/// true if the HashMap has at least one key-value pair, false if empty
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_not_empty;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// assert!(!umt_is_not_empty(&map));
///
/// map.insert("a", 1);
/// assert!(umt_is_not_empty(&map));
/// ```
#[inline]
pub fn umt_is_not_empty<K, V>(object: &HashMap<K, V>) -> bool {
    !object.is_empty()
}

/// Checks if a Vec is not empty
///
/// # Arguments
/// * `vec` - The Vec to check
///
/// # Returns
/// true if the Vec has at least one element, false if empty
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_not_empty_vec;
///
/// assert!(!umt_is_not_empty_vec(&Vec::<i32>::new()));
/// assert!(umt_is_not_empty_vec(&vec![1, 2, 3]));
/// ```
#[inline]
pub fn umt_is_not_empty_vec<T>(vec: &[T]) -> bool {
    !vec.is_empty()
}

/// Checks if a string is not empty
///
/// # Arguments
/// * `s` - The string to check
///
/// # Returns
/// true if the string has at least one character, false if empty
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_not_empty_str;
///
/// assert!(!umt_is_not_empty_str(""));
/// assert!(umt_is_not_empty_str("hello"));
/// ```
#[inline]
pub fn umt_is_not_empty_str(s: &str) -> bool {
    !s.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_empty_map() {
        let mut map: HashMap<&str, i32> = HashMap::new();
        assert!(!umt_is_not_empty(&map));

        map.insert("a", 1);
        assert!(umt_is_not_empty(&map));
    }

    #[test]
    fn test_is_not_empty_vec() {
        assert!(!umt_is_not_empty_vec(&Vec::<i32>::new()));
        assert!(umt_is_not_empty_vec(&vec![1, 2, 3]));
    }

    #[test]
    fn test_is_not_empty_str() {
        assert!(!umt_is_not_empty_str(""));
        assert!(umt_is_not_empty_str("hello"));
    }
}
