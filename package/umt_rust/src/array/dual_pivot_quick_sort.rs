use super::compare_function_default::umt_compare_function_default;
use super::sorting_helpers::{apply_insertion_sort_if_needed, validate_range};

const DEFAULT_INSERTION_SORT_THRESHOLD: usize = 10;

/// Result of partitioning with dual pivots.
struct PartitionResult {
    left_pivot_index: usize,
    right_pivot_index: usize,
}

/// Get the median of three elements in the array.
fn median_of_three<T, F>(array: &[T], a: usize, b: usize, c: usize, compare: &F) -> usize
where
    T: Clone,
    F: Fn(&T, &T) -> i32,
{
    // Create a vector of (index, value) pairs and sort by value
    let mut values = [
        (a, array[a].clone()),
        (b, array[b].clone()),
        (c, array[c].clone()),
    ];
    values.sort_by(|(_, va), (_, vb)| {
        let cmp = compare(va, vb);
        if cmp < 0 {
            std::cmp::Ordering::Less
        } else if cmp > 0 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    values[1].0
}

/// Select dual pivots and partition the array into three parts.
fn partition<T, F>(array: &mut [T], low: usize, high: usize, compare: &F) -> PartitionResult
where
    T: Clone,
    F: Fn(&T, &T) -> i32,
{
    // Select pivot candidates
    let length = high - low;
    let gap = (length / 3).max(1);

    // Find optimal pivot positions
    let left_pivot_index =
        median_of_three(array, low, low + gap, (low + 2 * gap).min(high), compare);

    let right_pivot_index = median_of_three(
        array,
        low.max(high.saturating_sub(2 * gap)),
        high.saturating_sub(gap),
        high,
        compare,
    );

    // Move pivots to ends
    if left_pivot_index != low {
        array.swap(low, left_pivot_index);
    }
    if right_pivot_index != high {
        array.swap(high, right_pivot_index);
    }

    // Swap pivots if needed
    if compare(&array[low], &array[high]) > 0 {
        array.swap(low, high);
    }

    // Initialize partition boundaries
    let mut left = low + 1;
    let mut right = high - 1;
    let mut current = left;

    // Partition into three parts
    while current <= right {
        // Handle elements smaller than left pivot
        if compare(&array[current], &array[low]) < 0 {
            array.swap(current, left);
            left += 1;
        }
        // Handle elements larger than right pivot
        else if compare(&array[current], &array[high]) > 0 {
            // Find suitable position from right
            while current < right && compare(&array[right], &array[high]) > 0 {
                right -= 1;
            }
            array.swap(current, right);
            right -= 1;
            // Check if swapped element is smaller than left pivot
            if compare(&array[current], &array[low]) < 0 {
                array.swap(current, left);
                left += 1;
            }
        }
        current += 1;
    }

    // Move pivots to their final positions
    left -= 1;
    right += 1;
    array.swap(low, left);
    array.swap(high, right);

    PartitionResult {
        left_pivot_index: left,
        right_pivot_index: right,
    }
}

/// Internal implementation of dual-pivot quicksort.
fn sort_range<T, F>(
    array: &mut [T],
    start: usize,
    end: usize,
    compare: &F,
    insertion_sort_threshold: usize,
) where
    T: Clone,
    F: Fn(&T, &T) -> i32,
{
    if start >= end {
        return;
    }

    if apply_insertion_sort_if_needed(array, start, end, compare, insertion_sort_threshold) {
        return;
    }

    // Get partition indices
    let PartitionResult {
        left_pivot_index,
        right_pivot_index,
    } = partition(array, start, end, compare);

    // Sort left partition
    if left_pivot_index > 0 && start < left_pivot_index {
        sort_range(
            array,
            start,
            left_pivot_index - 1,
            compare,
            insertion_sort_threshold,
        );
    }

    // Sort middle partition
    if right_pivot_index > left_pivot_index + 1 {
        sort_range(
            array,
            left_pivot_index + 1,
            right_pivot_index - 1,
            compare,
            insertion_sort_threshold,
        );
    }

    // Sort right partition
    if right_pivot_index < end {
        sort_range(
            array,
            right_pivot_index + 1,
            end,
            compare,
            insertion_sort_threshold,
        );
    }
}

/// Sort array using dual-pivot quicksort algorithm.
/// More efficient than traditional quicksort for arrays with many duplicate values.
///
/// # Arguments
///
/// * `array` - Array to be sorted
/// * `compare` - Optional comparison function
/// * `start_index` - Optional starting index (default: 0)
/// * `end_index` - Optional ending index (default: array.length - 1)
/// * `insertion_sort_threshold` - Optional threshold for insertion sort (default: 10)
///
/// # Returns
///
/// Sorted array
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_dual_pivot_quick_sort;
///
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
/// assert_eq!(
///     umt_dual_pivot_quick_sort(&arr, None, None, None, None),
///     vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]
/// );
/// ```
pub fn umt_dual_pivot_quick_sort<T: Clone + PartialOrd>(
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
        sort_range(
            &mut result,
            validated.start_index,
            validated.end_index,
            &compare_fn,
            threshold,
        );
    }

    result
}
