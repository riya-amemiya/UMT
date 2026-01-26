use std::collections::HashSet;
use std::hash::Hash;

/// Removes duplicate values from an array based on a selector function.
///
/// # Arguments
///
/// * `array` - The array to process
/// * `selector` - Function that returns the value to compare for uniqueness
///
/// # Returns
///
/// A new vector with unique values based on the selector
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_uniq_by;
///
/// let arr = vec![1.1, 1.2, 2.1, 2.2, 3.1];
/// let result = umt_uniq_by(&arr, |x| (*x as i32));
/// assert_eq!(result, vec![1.1, 2.1, 3.1]);
/// ```
pub fn umt_uniq_by<T, K, F>(array: &[T], selector: F) -> Vec<T>
where
    T: Clone,
    K: Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for item in array {
        let key = selector(item);
        if seen.insert(key) {
            result.push(item.clone());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniq_by_floor() {
        let arr = vec![1.1, 1.2, 2.1, 2.2, 3.1];
        let result = umt_uniq_by(&arr, |x| (*x as i32));
        assert_eq!(result, vec![1.1, 2.1, 3.1]);
    }

    #[test]
    fn test_uniq_by_string_length() {
        let arr = vec!["one", "two", "three", "four", "five"];
        let result = umt_uniq_by(&arr, |s| s.len());
        assert_eq!(result, vec!["one", "three", "four"]);
    }

    #[test]
    fn test_uniq_by_first_char() {
        let arr = vec!["apple", "apricot", "banana", "blueberry", "cherry"];
        let result = umt_uniq_by(&arr, |s| s.chars().next().unwrap_or('\0'));
        assert_eq!(result, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_uniq_by_struct_field() {
        #[derive(Clone, Debug, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }

        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            Person {
                name: "Charlie".to_string(),
                age: 25,
            },
        ];

        let result = umt_uniq_by(&people, |p| p.age);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "Alice");
        assert_eq!(result[1].name, "Charlie");
    }

    #[test]
    fn test_uniq_by_empty() {
        let arr: Vec<i32> = vec![];
        let result = umt_uniq_by(&arr, |x| *x);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_uniq_by_all_same_key() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = umt_uniq_by(&arr, |_| 0);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_uniq_by_all_different_keys() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = umt_uniq_by(&arr, |x| *x);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_uniq_by_modulo() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = umt_uniq_by(&arr, |x| x % 3);
        assert_eq!(result, vec![1, 2, 3]);
    }
}
