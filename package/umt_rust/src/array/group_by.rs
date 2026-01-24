use std::collections::HashMap;
use std::hash::Hash;

/// Groups elements of an array based on a given iteratee function.
///
/// # Arguments
///
/// * `array` - Array to group
/// * `iteratee` - Function to determine the group key for each element
///
/// # Returns
///
/// HashMap with grouped elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_group_by;
///
/// let arr = vec![6.1, 4.2, 6.3];
/// let result = umt_group_by(&arr, |x| (*x as i32));
/// assert_eq!(result.get(&4), Some(&vec![4.2]));
/// assert_eq!(result.get(&6), Some(&vec![6.1, 6.3]));
/// ```
pub fn umt_group_by<T, K, F>(array: &[T], iteratee: F) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut result: HashMap<K, Vec<T>> = HashMap::new();

    for item in array {
        let key = iteratee(item);
        result.entry(key).or_default().push(item.clone());
    }

    result
}

/// Groups elements of an array with index access in the iteratee.
///
/// # Arguments
///
/// * `array` - Array to group
/// * `iteratee` - Function to determine the group key (receives value and index)
///
/// # Returns
///
/// HashMap with grouped elements
pub fn umt_group_by_indexed<T, K, F>(array: &[T], iteratee: F) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Hash + Eq,
    F: Fn(&T, usize) -> K,
{
    let mut result: HashMap<K, Vec<T>> = HashMap::new();

    for (index, item) in array.iter().enumerate() {
        let key = iteratee(item, index);
        result.entry(key).or_default().push(item.clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_by_floor() {
        let arr = vec![6.1, 4.2, 6.3];
        let result = umt_group_by(&arr, |x| (*x as i32));

        assert_eq!(result.get(&4), Some(&vec![4.2]));
        assert_eq!(result.get(&6), Some(&vec![6.1, 6.3]));
    }

    #[test]
    fn test_group_by_string_length() {
        let arr = vec!["one", "two", "three"];
        let result = umt_group_by(&arr, |s| s.len());

        assert_eq!(result.get(&3), Some(&vec!["one", "two"]));
        assert_eq!(result.get(&5), Some(&vec!["three"]));
    }

    #[test]
    fn test_group_by_first_char() {
        let arr = vec!["apple", "apricot", "banana", "blueberry", "cherry"];
        let result = umt_group_by(&arr, |s| s.chars().next().unwrap_or('\0'));

        assert_eq!(result.get(&'a'), Some(&vec!["apple", "apricot"]));
        assert_eq!(result.get(&'b'), Some(&vec!["banana", "blueberry"]));
        assert_eq!(result.get(&'c'), Some(&vec!["cherry"]));
    }

    #[test]
    fn test_group_by_empty() {
        let arr: Vec<i32> = vec![];
        let result = umt_group_by(&arr, |x| *x);
        assert!(result.is_empty());
    }

    #[test]
    fn test_group_by_single_group() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = umt_group_by(&arr, |_| "all");
        assert_eq!(result.get(&"all"), Some(&vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_group_by_modulo() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = umt_group_by(&arr, |x| x % 3);

        assert_eq!(result.get(&0), Some(&vec![3, 6, 9]));
        assert_eq!(result.get(&1), Some(&vec![1, 4, 7]));
        assert_eq!(result.get(&2), Some(&vec![2, 5, 8]));
    }

    #[test]
    fn test_group_by_indexed() {
        let arr = vec!["a", "b", "c", "d", "e"];
        let result = umt_group_by_indexed(&arr, |_, idx| idx % 2);

        assert_eq!(result.get(&0), Some(&vec!["a", "c", "e"]));
        assert_eq!(result.get(&1), Some(&vec!["b", "d"]));
    }

    #[test]
    fn test_group_by_struct() {
        #[derive(Clone, Debug, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }

        let people = vec![
            Person { name: "Alice".to_string(), age: 30 },
            Person { name: "Bob".to_string(), age: 25 },
            Person { name: "Charlie".to_string(), age: 30 },
        ];

        let result = umt_group_by(&people, |p| p.age);

        assert_eq!(result.get(&30).unwrap().len(), 2);
        assert_eq!(result.get(&25).unwrap().len(), 1);
    }
}
