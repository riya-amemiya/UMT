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
    if high >= low && high - low + 1 <= insertion_sort_threshold {
        insertion_sort_range(array, compare, low, high);
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_compare<T: PartialOrd>(a: &T, b: &T) -> i32 {
        if a > b {
            1
        } else if a < b {
            -1
        } else {
            0
        }
    }

    #[test]
    fn test_apply_insertion_sort_if_needed_small() {
        let mut arr = vec![5, 2, 8, 1, 9];
        let applied = apply_insertion_sort_if_needed(&mut arr, 0, 4, &default_compare, 10);
        assert!(applied);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_apply_insertion_sort_if_needed_large() {
        let mut arr = vec![5, 2, 8, 1, 9, 3, 7, 4, 6, 0, 10];
        let original = arr.clone();
        let applied = apply_insertion_sort_if_needed(&mut arr, 0, 10, &default_compare, 5);
        assert!(!applied);
        assert_eq!(arr, original);
    }

    #[test]
    fn test_apply_insertion_sort_if_needed_exact_threshold() {
        let mut arr = vec![5, 2, 8, 1, 9];
        let applied = apply_insertion_sort_if_needed(&mut arr, 0, 4, &default_compare, 5);
        assert!(applied);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_apply_insertion_sort_if_needed_partial_range() {
        let mut arr = vec![5, 2, 8, 1, 9];
        let applied = apply_insertion_sort_if_needed(&mut arr, 1, 3, &default_compare, 5);
        assert!(applied);
        assert_eq!(arr, vec![5, 1, 2, 8, 9]);
    }
}
