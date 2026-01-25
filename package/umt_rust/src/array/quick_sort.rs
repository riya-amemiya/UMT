use super::compare_function_default::umt_compare_function_default;
use super::sorting_helpers::{apply_insertion_sort_if_needed, validate_range};

const DEFAULT_INSERTION_SORT_THRESHOLD: usize = 10;

/// Finds the median value among three elements in the array.
fn median_of_three<T, F>(array: &[T], a: usize, b: usize, c: usize, compare: &F) -> T
where
    T: Clone,
    F: Fn(&T, &T) -> i32,
{
    let ab = compare(&array[a], &array[b]);
    if ab < 0 {
        let bc = compare(&array[b], &array[c]);
        if bc < 0 {
            return array[b].clone();
        }
        if compare(&array[a], &array[c]) < 0 {
            return array[c].clone();
        }
        return array[a].clone();
    }
    let ac = compare(&array[a], &array[c]);
    if ac < 0 {
        return array[a].clone();
    }
    if compare(&array[b], &array[c]) < 0 {
        return array[c].clone();
    }
    array[b].clone()
}

/// Partitions the array around a pivot element using median-of-three strategy.
fn partition<T, F>(array: &mut [T], low: usize, high: usize, compare: &F) -> usize
where
    T: Clone,
    F: Fn(&T, &T) -> i32,
{
    let pivot = median_of_three(array, low, (low + high) / 2, high, compare);
    let mut left = low;
    let mut right = high;

    loop {
        while compare(&array[left], &pivot) < 0 {
            left += 1;
        }
        while compare(&array[right], &pivot) > 0 {
            right -= 1;
        }

        if left >= right {
            return right;
        }

        array.swap(left, right);
        left += 1;
        right = right.saturating_sub(1);
    }
}

/// Internal implementation of quicksort with tail-call optimization.
fn sort_impl<T, F>(
    array: &mut [T],
    low_init: usize,
    high_init: usize,
    compare: &F,
    insertion_sort_threshold: usize,
) where
    T: Clone,
    F: Fn(&T, &T) -> i32,
{
    let mut low = low_init;
    let mut high = high_init;

    while low < high {
        if apply_insertion_sort_if_needed(array, low, high, compare, insertion_sort_threshold) {
            return;
        }

        let pivot_index = partition(array, low, high, compare);

        if pivot_index.saturating_sub(low) < high.saturating_sub(pivot_index) {
            if pivot_index > 0 {
                sort_impl(array, low, pivot_index, compare, insertion_sort_threshold);
            }
            low = pivot_index + 1;
        } else {
            sort_impl(
                array,
                pivot_index + 1,
                high,
                compare,
                insertion_sort_threshold,
            );
            if pivot_index == 0 {
                break;
            }
            high = pivot_index;
        }
    }
}

/// Sorts an array using a hybrid algorithm combining QuickSort and InsertionSort.
///
/// # Arguments
///
/// * `array` - Array to sort
/// * `compare` - Optional comparison function
/// * `start_index` - Starting index for the sort range (default: 0)
/// * `end_index` - Ending index for the sort range (default: array.length - 1)
/// * `insertion_sort_threshold` - Threshold for switching to insertion sort (default: 10)
///
/// # Returns
///
/// Sorted array
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_quick_sort;
///
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// assert_eq!(umt_quick_sort(&arr, None, None, None, None), vec![1, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn umt_quick_sort<T: Clone + PartialOrd>(
    array: &[T],
    compare: Option<fn(&T, &T) -> i32>,
    start_index: Option<usize>,
    end_index: Option<usize>,
    insertion_sort_threshold: Option<usize>,
) -> Vec<T> {
    if array.is_empty() {
        return vec![];
    }

    let mut result = array.to_vec();
    let compare_fn = compare.unwrap_or(umt_compare_function_default);
    let threshold = insertion_sort_threshold.unwrap_or(DEFAULT_INSERTION_SORT_THRESHOLD);

    let start = start_index.unwrap_or(0);
    let end = end_index.unwrap_or(array.len() - 1);

    let validated = validate_range(array.len(), start, end);

    if validated.should_sort {
        sort_impl(
            &mut result,
            validated.start_index,
            validated.end_index,
            &compare_fn,
            threshold,
        );
    }

    result
}

/// Sorts an array in place using QuickSort.
///
/// # Arguments
///
/// * `array` - Mutable array to sort
/// * `compare` - Optional comparison function
pub fn umt_quick_sort_in_place<T: Clone + PartialOrd>(
    array: &mut [T],
    compare: Option<fn(&T, &T) -> i32>,
) {
    if array.len() <= 1 {
        return;
    }

    let compare_fn = compare.unwrap_or(umt_compare_function_default);
    sort_impl(
        array,
        0,
        array.len() - 1,
        &compare_fn,
        DEFAULT_INSERTION_SORT_THRESHOLD,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_basic() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(
            umt_quick_sort(&arr, None, None, None, None),
            vec![1, 1, 2, 3, 4, 5, 6, 9]
        );
    }

    #[test]
    fn test_quick_sort_empty() {
        let arr: Vec<i32> = vec![];
        assert_eq!(
            umt_quick_sort(&arr, None, None, None, None),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_quick_sort_single() {
        let arr = vec![42];
        assert_eq!(umt_quick_sort(&arr, None, None, None, None), vec![42]);
    }

    #[test]
    fn test_quick_sort_already_sorted() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(
            umt_quick_sort(&arr, None, None, None, None),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_quick_sort_reverse() {
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(
            umt_quick_sort(&arr, None, None, None, None),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_quick_sort_with_custom_compare() {
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
            umt_quick_sort(&arr, Some(descending), None, None, None),
            vec![5, 4, 3, 2, 1]
        );
    }

    #[test]
    fn test_quick_sort_strings() {
        let arr = vec!["banana", "apple", "cherry"];
        assert_eq!(
            umt_quick_sort(&arr, None, None, None, None),
            vec!["apple", "banana", "cherry"]
        );
    }

    #[test]
    fn test_quick_sort_does_not_mutate() {
        let arr = vec![3, 1, 4, 1, 5];
        let _ = umt_quick_sort(&arr, None, None, None, None);
        assert_eq!(arr, vec![3, 1, 4, 1, 5]);
    }

    #[test]
    fn test_quick_sort_in_place() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        umt_quick_sort_in_place(&mut arr, None);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_quick_sort_partial_range() {
        let arr = vec![5, 3, 1, 4, 2];
        // Sort only indices 1 to 3
        let result = umt_quick_sort(&arr, None, Some(1), Some(3), None);
        assert_eq!(result, vec![5, 1, 3, 4, 2]);
    }

    #[test]
    fn test_quick_sort_large() {
        let arr: Vec<i32> = (0..1000).rev().collect();
        let sorted = umt_quick_sort(&arr, None, None, None, None);
        assert_eq!(sorted, (0..1000).collect::<Vec<i32>>());
    }

    #[test]
    fn test_quick_sort_duplicates() {
        let arr = vec![3, 3, 1, 1, 2, 2];
        assert_eq!(
            umt_quick_sort(&arr, None, None, None, None),
            vec![1, 1, 2, 2, 3, 3]
        );
    }
}
