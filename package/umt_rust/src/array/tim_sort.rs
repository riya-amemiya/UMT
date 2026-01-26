use super::compare_function_default::umt_compare_function_default;
use super::sorting_helpers::insertion_sort_range;

const MIN_RUN: usize = 32;

/// Merges two sorted portions of the array.
fn merge<T, F>(array: &mut [T], start: usize, mid: usize, end: usize, compare: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> i32,
{
    let left: Vec<T> = array[start..=mid].to_vec();
    let right: Vec<T> = array[(mid + 1)..=end].to_vec();

    let mut left_index = 0;
    let mut right_index = 0;
    let mut array_index = start;

    while left_index < left.len() && right_index < right.len() {
        if compare(&left[left_index], &right[right_index]) <= 0 {
            array[array_index] = left[left_index].clone();
            left_index += 1;
        } else {
            array[array_index] = right[right_index].clone();
            right_index += 1;
        }
        array_index += 1;
    }

    while left_index < left.len() {
        array[array_index] = left[left_index].clone();
        left_index += 1;
        array_index += 1;
    }

    while right_index < right.len() {
        array[array_index] = right[right_index].clone();
        right_index += 1;
        array_index += 1;
    }
}

/// Calculates the minimum length of a run for the given input size.
fn get_min_run_length(mut n: usize) -> usize {
    let mut r = 0;
    while n >= MIN_RUN {
        r |= n & 1;
        n >>= 1;
    }
    n + r
}

/// Implementation of the TimSort algorithm, which combines the best features of
/// insertion sort and merge sort. It provides a stable sort with O(n log n)
/// worst-case time complexity.
///
/// # Arguments
///
/// * `array` - Array to sort
/// * `compare` - Optional comparison function
/// * `start` - Starting index for the sort range (default: 0)
/// * `end` - Ending index for the sort range (default: array.length - 1)
///
/// # Returns
///
/// Sorted array
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_tim_sort;
///
/// let arr = vec![3, 1, 4, 1, 5];
/// assert_eq!(umt_tim_sort(&arr, None, None, None), vec![1, 1, 3, 4, 5]);
/// ```
pub fn umt_tim_sort<T: Clone + PartialOrd>(
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

    let start_idx = start.unwrap_or(0).min(array.len() - 1);
    let end_idx = end.unwrap_or(array.len() - 1).min(array.len() - 1);

    if start_idx >= end_idx {
        return result;
    }

    let n = end_idx - start_idx + 1;
    let min_run = get_min_run_length(n);

    // Sort individual runs using insertion sort
    let mut run_start = start_idx;
    while run_start <= end_idx {
        let run_end = (run_start + min_run - 1).min(end_idx);
        insertion_sort_range(&mut result, &compare_fn, run_start, run_end);
        run_start += min_run;
    }

    // Merge runs
    let mut size = min_run;
    while size < n {
        let mut left = start_idx;
        while left <= end_idx {
            let mid = left + size - 1;
            let right = (left + 2 * size - 1).min(end_idx);

            if mid < right {
                merge(&mut result, left, mid, right, &compare_fn);
            }
            left += 2 * size;
        }
        size *= 2;
    }

    result
}

/// Sort an array in place using TimSort.
///
/// # Arguments
///
/// * `array` - Mutable array to sort
/// * `compare` - Optional comparison function
pub fn umt_tim_sort_in_place<T: Clone + PartialOrd>(
    array: &mut [T],
    compare: Option<fn(&T, &T) -> i32>,
) {
    if array.len() <= 1 {
        return;
    }

    let compare_fn = compare.unwrap_or(umt_compare_function_default);
    let n = array.len();
    let min_run = get_min_run_length(n);

    // Sort individual runs using insertion sort
    let mut run_start = 0;
    while run_start < n {
        let run_end = (run_start + min_run - 1).min(n - 1);
        insertion_sort_range(array, &compare_fn, run_start, run_end);
        run_start += min_run;
    }

    // Merge runs
    let mut size = min_run;
    while size < n {
        let mut left = 0;
        while left < n {
            let mid = left + size - 1;
            let right = (left + 2 * size - 1).min(n - 1);

            if mid < right {
                merge(array, left, mid, right, &compare_fn);
            }
            left += 2 * size;
        }
        size *= 2;
    }
}
