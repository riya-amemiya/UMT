use super::compare_function_default::umt_compare_function_default;
use super::sorting_helpers::insertion_sort_range;

/// Sort an array using insertion sort algorithm.
///
/// # Arguments
///
/// * `array` - Array to sort
/// * `compare` - Optional comparison function
/// * `start` - Starting index for sorting (inclusive)
/// * `end` - Ending index for sorting (inclusive)
///
/// # Returns
///
/// Sorted array
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_insertion_sort;
///
/// let arr = vec![4, 2, 7, 1, 3];
/// assert_eq!(umt_insertion_sort(&arr, None, None, None), vec![1, 2, 3, 4, 7]);
/// ```
pub fn umt_insertion_sort<T: Clone + PartialOrd>(
    array: &[T],
    compare: Option<fn(&T, &T) -> i32>,
    start: Option<usize>,
    end: Option<usize>,
) -> Vec<T> {
    if array.is_empty() {
        return vec![];
    }

    let mut result = array.to_vec();
    let compare_fn = compare.unwrap_or(umt_compare_function_default);
    let start_idx = start.unwrap_or(0);
    let end_idx = end.unwrap_or(array.len() - 1).min(array.len() - 1);

    insertion_sort_range(&mut result, &compare_fn, start_idx, end_idx);

    result
}

/// Sort an array in place using insertion sort algorithm.
///
/// # Arguments
///
/// * `array` - Mutable array to sort
/// * `compare` - Optional comparison function
/// * `start` - Starting index for sorting (inclusive)
/// * `end` - Ending index for sorting (inclusive)
pub fn umt_insertion_sort_in_place<T: Clone + PartialOrd>(
    array: &mut [T],
    compare: Option<fn(&T, &T) -> i32>,
    start: Option<usize>,
    end: Option<usize>,
) {
    if array.is_empty() {
        return;
    }

    let compare_fn = compare.unwrap_or(umt_compare_function_default);
    let start_idx = start.unwrap_or(0);
    let end_idx = end.unwrap_or(array.len() - 1).min(array.len() - 1);

    insertion_sort_range(array, &compare_fn, start_idx, end_idx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_basic() {
        let arr = vec![4, 2, 7, 1, 3];
        assert_eq!(
            umt_insertion_sort(&arr, None, None, None),
            vec![1, 2, 3, 4, 7]
        );
    }

    #[test]
    fn test_insertion_sort_empty() {
        let arr: Vec<i32> = vec![];
        assert_eq!(
            umt_insertion_sort(&arr, None, None, None),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_insertion_sort_single() {
        let arr = vec![42];
        assert_eq!(umt_insertion_sort(&arr, None, None, None), vec![42]);
    }

    #[test]
    fn test_insertion_sort_already_sorted() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(
            umt_insertion_sort(&arr, None, None, None),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_insertion_sort_reverse() {
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(
            umt_insertion_sort(&arr, None, None, None),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_insertion_sort_partial_range() {
        let arr = vec![4, 2, 7, 1, 3];
        // Sort only indices 1 to 3 (2, 7, 1)
        assert_eq!(
            umt_insertion_sort(&arr, None, Some(1), Some(3)),
            vec![4, 1, 2, 7, 3]
        );
    }

    #[test]
    fn test_insertion_sort_with_custom_compare() {
        let arr = vec![1, 2, 3, 4, 5];
        // Sort in descending order
        let descending = |a: &i32, b: &i32| -> i32 {
            if a < b {
                1
            } else if a > b {
                -1
            } else {
                0
            }
        };
        assert_eq!(
            umt_insertion_sort(&arr, Some(descending), None, None),
            vec![5, 4, 3, 2, 1]
        );
    }

    #[test]
    fn test_insertion_sort_strings() {
        let arr = vec!["banana", "apple", "cherry"];
        assert_eq!(
            umt_insertion_sort(&arr, None, None, None),
            vec!["apple", "banana", "cherry"]
        );
    }

    #[test]
    fn test_insertion_sort_does_not_mutate() {
        let arr = vec![4, 2, 7, 1, 3];
        let _ = umt_insertion_sort(&arr, None, None, None);
        assert_eq!(arr, vec![4, 2, 7, 1, 3]);
    }

    #[test]
    fn test_insertion_sort_in_place() {
        let mut arr = vec![4, 2, 7, 1, 3];
        umt_insertion_sort_in_place(&mut arr, None, None, None);
        assert_eq!(arr, vec![1, 2, 3, 4, 7]);
    }

    #[test]
    fn test_insertion_sort_duplicates() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        assert_eq!(
            umt_insertion_sort(&arr, None, None, None),
            vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]
        );
    }
}
