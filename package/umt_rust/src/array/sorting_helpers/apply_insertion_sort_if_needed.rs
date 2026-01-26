use super::insertion_sort_range::insertion_sort_range;

/// Checks if a partition is small enough to apply insertion sort and applies it if so.
///
/// # Arguments
///
/// * `array` - The array containing the partition
/// * `low` - The starting index of the partition
/// * `high` - The ending index of the partition
/// * `compare` - The function to compare elements
/// * `insertion_sort_threshold` - The size threshold for switching to insertion sort
///
/// # Returns
///
/// `true` if insertion sort was applied, `false` otherwise.
pub fn apply_insertion_sort_if_needed<T, F>(
    array: &mut [T],
    low: usize,
    high: usize,
    compare: &F,
    insertion_sort_threshold: usize,
) -> bool
where
    F: Fn(&T, &T) -> i32,
{
    if high >= low && high - low < insertion_sort_threshold {
        insertion_sort_range(array, compare, low, high);
        return true;
    }
    false
}
