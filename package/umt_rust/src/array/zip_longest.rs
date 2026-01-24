/// Combines two arrays by padding shorter arrays with None values
/// to match the length of the longest array.
///
/// # Arguments
///
/// * `array1` - First array
/// * `array2` - Second array
///
/// # Returns
///
/// Vector of tuples with Option-wrapped elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_zip_longest;
///
/// let a = vec![1, 2];
/// let b = vec!["a"];
/// assert_eq!(
///     umt_zip_longest(&a, &b),
///     vec![(Some(1), Some("a")), (Some(2), None)]
/// );
/// ```
pub fn umt_zip_longest<T: Clone, U: Clone>(
    array1: &[T],
    array2: &[U],
) -> Vec<(Option<T>, Option<U>)> {
    let max_length = array1.len().max(array2.len());
    (0..max_length)
        .map(|i| (array1.get(i).cloned(), array2.get(i).cloned()))
        .collect()
}

/// Combines three arrays by padding shorter arrays with None values.
///
/// # Arguments
///
/// * `array1` - First array
/// * `array2` - Second array
/// * `array3` - Third array
///
/// # Returns
///
/// Vector of tuples with Option-wrapped elements
pub fn umt_zip_longest3<T: Clone, U: Clone, V: Clone>(
    array1: &[T],
    array2: &[U],
    array3: &[V],
) -> Vec<(Option<T>, Option<U>, Option<V>)> {
    let max_length = array1.len().max(array2.len()).max(array3.len());
    (0..max_length)
        .map(|i| {
            (
                array1.get(i).cloned(),
                array2.get(i).cloned(),
                array3.get(i).cloned(),
            )
        })
        .collect()
}

/// Combines multiple arrays of the same type by padding shorter arrays with None values.
///
/// # Arguments
///
/// * `arrays` - Slice of arrays to combine
///
/// # Returns
///
/// Vector of vectors with Option-wrapped elements
pub fn umt_zip_longest_many<T: Clone>(arrays: &[&[T]]) -> Vec<Vec<Option<T>>> {
    if arrays.is_empty() {
        return vec![];
    }

    let max_length = arrays.iter().map(|a| a.len()).max().unwrap_or(0);
    (0..max_length)
        .map(|i| arrays.iter().map(|arr| arr.get(i).cloned()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip_longest_basic() {
        let a = vec![1, 2];
        let b = vec!["a"];
        assert_eq!(
            umt_zip_longest(&a, &b),
            vec![(Some(1), Some("a")), (Some(2), None)]
        );
    }

    #[test]
    fn test_zip_longest_first_shorter() {
        let a = vec![1];
        let b = vec!["a", "b"];
        assert_eq!(
            umt_zip_longest(&a, &b),
            vec![(Some(1), Some("a")), (None, Some("b"))]
        );
    }

    #[test]
    fn test_zip_longest_equal_length() {
        let a = vec![1, 2];
        let b = vec!["a", "b"];
        assert_eq!(
            umt_zip_longest(&a, &b),
            vec![(Some(1), Some("a")), (Some(2), Some("b"))]
        );
    }

    #[test]
    fn test_zip_longest_empty_arrays() {
        let a: Vec<i32> = vec![];
        let b: Vec<&str> = vec![];
        assert_eq!(
            umt_zip_longest(&a, &b),
            Vec::<(Option<i32>, Option<&str>)>::new()
        );
    }

    #[test]
    fn test_zip_longest_one_empty() {
        let a = vec![1, 2, 3];
        let b: Vec<&str> = vec![];
        assert_eq!(
            umt_zip_longest(&a, &b),
            vec![(Some(1), None), (Some(2), None), (Some(3), None)]
        );
    }

    #[test]
    fn test_zip_longest3() {
        let a = vec![1, 2];
        let b = vec!["a"];
        let c = vec![true, false, true];
        assert_eq!(
            umt_zip_longest3(&a, &b, &c),
            vec![
                (Some(1), Some("a"), Some(true)),
                (Some(2), None, Some(false)),
                (None, None, Some(true)),
            ]
        );
    }

    #[test]
    fn test_zip_longest_many() {
        let a = vec![1, 2];
        let b = vec![3, 4, 5];
        let c = vec![6];
        assert_eq!(
            umt_zip_longest_many(&[&a[..], &b[..], &c[..]]),
            vec![
                vec![Some(1), Some(3), Some(6)],
                vec![Some(2), Some(4), None],
                vec![None, Some(5), None],
            ]
        );
    }

    #[test]
    fn test_zip_longest_many_empty() {
        let arrays: &[&[i32]] = &[];
        assert_eq!(umt_zip_longest_many(arrays), Vec::<Vec<Option<i32>>>::new());
    }

    #[test]
    fn test_zip_longest_many_all_empty() {
        let a: Vec<i32> = vec![];
        let b: Vec<i32> = vec![];
        assert_eq!(
            umt_zip_longest_many(&[&a[..], &b[..]]),
            Vec::<Vec<Option<i32>>>::new()
        );
    }
}
