/// Creates a new vector by combining elements from two arrays at corresponding positions.
/// The length of the result is the minimum of the two input arrays.
///
/// # Arguments
///
/// * `array1` - First array
/// * `array2` - Second array
///
/// # Returns
///
/// Vector of tuples with combined elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_zip;
///
/// let a = vec![1, 2, 3];
/// let b = vec!["a", "b", "c"];
/// assert_eq!(umt_zip(&a, &b), vec![(1, "a"), (2, "b"), (3, "c")]);
/// ```
pub fn umt_zip<T: Clone, U: Clone>(array1: &[T], array2: &[U]) -> Vec<(T, U)> {
    let length = array1.len().min(array2.len());
    (0..length)
        .map(|i| (array1[i].clone(), array2[i].clone()))
        .collect()
}

/// Creates a new vector by combining elements from three arrays at corresponding positions.
/// The length of the result is the minimum of all input arrays.
///
/// # Arguments
///
/// * `array1` - First array
/// * `array2` - Second array
/// * `array3` - Third array
///
/// # Returns
///
/// Vector of tuples with combined elements
pub fn umt_zip3<T: Clone, U: Clone, V: Clone>(
    array1: &[T],
    array2: &[U],
    array3: &[V],
) -> Vec<(T, U, V)> {
    let length = array1.len().min(array2.len()).min(array3.len());
    (0..length)
        .map(|i| (array1[i].clone(), array2[i].clone(), array3[i].clone()))
        .collect()
}

/// Creates a new vector by combining elements from multiple arrays of the same type.
/// The length of the result is the minimum of all input arrays.
///
/// # Arguments
///
/// * `arrays` - Slice of arrays to combine
///
/// # Returns
///
/// Vector of vectors with combined elements
pub fn umt_zip_many<T: Clone>(arrays: &[&[T]]) -> Vec<Vec<T>> {
    if arrays.is_empty() {
        return vec![];
    }

    let length = arrays.iter().map(|a| a.len()).min().unwrap_or(0);
    (0..length)
        .map(|i| arrays.iter().map(|arr| arr[i].clone()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip_basic() {
        let a = vec![1, 2, 3];
        let b = vec!["a", "b", "c"];
        assert_eq!(umt_zip(&a, &b), vec![(1, "a"), (2, "b"), (3, "c")]);
    }

    #[test]
    fn test_zip_different_lengths_first_shorter() {
        let a = vec![1, 2];
        let b = vec!["a", "b", "c"];
        assert_eq!(umt_zip(&a, &b), vec![(1, "a"), (2, "b")]);
    }

    #[test]
    fn test_zip_different_lengths_second_shorter() {
        let a = vec![1, 2, 3];
        let b = vec!["a", "b"];
        assert_eq!(umt_zip(&a, &b), vec![(1, "a"), (2, "b")]);
    }

    #[test]
    fn test_zip_empty_arrays() {
        let a: Vec<i32> = vec![];
        let b: Vec<&str> = vec![];
        assert_eq!(umt_zip(&a, &b), Vec::<(i32, &str)>::new());
    }

    #[test]
    fn test_zip_one_empty() {
        let a = vec![1, 2, 3];
        let b: Vec<&str> = vec![];
        assert_eq!(umt_zip(&a, &b), Vec::<(i32, &str)>::new());
    }

    #[test]
    fn test_zip_same_type() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        assert_eq!(umt_zip(&a, &b), vec![(1, 4), (2, 5), (3, 6)]);
    }

    #[test]
    fn test_zip3() {
        let a = vec![1, 2, 3];
        let b = vec!["a", "b", "c"];
        let c = vec![true, false, true];
        assert_eq!(
            umt_zip3(&a, &b, &c),
            vec![(1, "a", true), (2, "b", false), (3, "c", true)]
        );
    }

    #[test]
    fn test_zip3_different_lengths() {
        let a = vec![1, 2];
        let b = vec!["a", "b", "c"];
        let c = vec![true, false, true, false];
        assert_eq!(umt_zip3(&a, &b, &c), vec![(1, "a", true), (2, "b", false)]);
    }

    #[test]
    fn test_zip_many() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![7, 8, 9];
        assert_eq!(
            umt_zip_many(&[&a[..], &b[..], &c[..]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
    }

    #[test]
    fn test_zip_many_different_lengths() {
        let a = vec![1, 2];
        let b = vec![4, 5, 6];
        let c = vec![7, 8, 9, 10];
        assert_eq!(
            umt_zip_many(&[&a[..], &b[..], &c[..]]),
            vec![vec![1, 4, 7], vec![2, 5, 8]]
        );
    }

    #[test]
    fn test_zip_many_empty() {
        let arrays: &[&[i32]] = &[];
        assert_eq!(umt_zip_many(arrays), Vec::<Vec<i32>>::new());
    }
}
