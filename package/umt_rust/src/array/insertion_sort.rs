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
