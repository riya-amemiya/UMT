use crate::internal::rng;

/// Shuffles all elements in a 2D array while maintaining the row lengths.
///
/// # Arguments
///
/// * `array` - The 2D array to shuffle
///
/// # Returns
///
/// A new 2D vector with shuffled elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_shuffle_2d_array;
///
/// let arr = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
/// let shuffled = umt_shuffle_2d_array(&arr);
/// // Row lengths are preserved, but elements are shuffled across the entire array
/// assert_eq!(shuffled[0].len(), 2);
/// assert_eq!(shuffled[1].len(), 2);
/// assert_eq!(shuffled[2].len(), 2);
/// ```
pub fn umt_shuffle_2d_array<T: Clone>(array: &[Vec<T>]) -> Vec<Vec<T>> {
    if array.is_empty() {
        return vec![];
    }

    // Flatten the 2D array into 1D
    let mut flat_array: Vec<T> = Vec::new();
    for sub_array in array {
        flat_array.extend(sub_array.iter().cloned());
    }

    // Shuffle the flat array using Fisher-Yates
    for i in (1..flat_array.len()).rev() {
        let j = rng::random_range_usize(0, i);
        flat_array.swap(i, j);
    }

    // Reconstruct the 2D array with the same row lengths
    let mut result = Vec::new();
    let mut row_index = 0;

    for sub_array in array {
        let row_len = sub_array.len();
        let new_row = flat_array[row_index..row_index + row_len].to_vec();
        result.push(new_row);
        row_index += row_len;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_shuffle_2d_maintains_row_lengths() {
        let arr = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
        let shuffled = umt_shuffle_2d_array(&arr);

        assert_eq!(shuffled.len(), 3);
        assert_eq!(shuffled[0].len(), 2);
        assert_eq!(shuffled[1].len(), 3);
        assert_eq!(shuffled[2].len(), 1);
    }

    #[test]
    fn test_shuffle_2d_maintains_elements() {
        let arr = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let shuffled = umt_shuffle_2d_array(&arr);

        let original_elements: HashSet<_> = arr.iter().flatten().cloned().collect();
        let shuffled_elements: HashSet<_> = shuffled.iter().flatten().cloned().collect();

        assert_eq!(original_elements, shuffled_elements);
    }

    #[test]
    fn test_shuffle_2d_empty() {
        let arr: Vec<Vec<i32>> = vec![];
        let shuffled = umt_shuffle_2d_array(&arr);
        assert!(shuffled.is_empty());
    }

    #[test]
    fn test_shuffle_2d_single_row() {
        let arr = vec![vec![1, 2, 3, 4, 5]];
        let shuffled = umt_shuffle_2d_array(&arr);

        assert_eq!(shuffled.len(), 1);
        assert_eq!(shuffled[0].len(), 5);

        let original_elements: HashSet<_> = arr[0].iter().cloned().collect();
        let shuffled_elements: HashSet<_> = shuffled[0].iter().cloned().collect();
        assert_eq!(original_elements, shuffled_elements);
    }

    #[test]
    fn test_shuffle_2d_single_column() {
        let arr = vec![vec![1], vec![2], vec![3], vec![4], vec![5]];
        let shuffled = umt_shuffle_2d_array(&arr);

        assert_eq!(shuffled.len(), 5);
        for row in &shuffled {
            assert_eq!(row.len(), 1);
        }

        let original_elements: HashSet<_> = arr.iter().flatten().cloned().collect();
        let shuffled_elements: HashSet<_> = shuffled.iter().flatten().cloned().collect();
        assert_eq!(original_elements, shuffled_elements);
    }

    #[test]
    fn test_shuffle_2d_empty_rows() {
        let arr = vec![vec![1, 2], vec![], vec![3, 4, 5]];
        let shuffled = umt_shuffle_2d_array(&arr);

        assert_eq!(shuffled.len(), 3);
        assert_eq!(shuffled[0].len(), 2);
        assert_eq!(shuffled[1].len(), 0);
        assert_eq!(shuffled[2].len(), 3);
    }

    #[test]
    fn test_shuffle_2d_strings() {
        let arr = vec![vec!["a", "b"], vec!["c", "d"], vec!["e", "f"]];
        let shuffled = umt_shuffle_2d_array(&arr);

        let original_elements: HashSet<_> = arr.iter().flatten().cloned().collect();
        let shuffled_elements: HashSet<_> = shuffled.iter().flatten().cloned().collect();
        assert_eq!(original_elements, shuffled_elements);
    }
}
