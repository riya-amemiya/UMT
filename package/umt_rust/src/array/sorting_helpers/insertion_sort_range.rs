/// Performs insertion sort on a range of the array in-place.
///
/// # Arguments
///
/// * `array` - The array to sort
/// * `compare` - Comparison function that returns negative if a < b, zero if a == b, positive if a > b
/// * `start` - Starting index (inclusive)
/// * `end` - Ending index (inclusive)
///
/// # Type Parameters
///
/// * `T` - The type of elements in the array
/// * `F` - The comparison function type
pub fn insertion_sort_range<T, F>(array: &mut [T], compare: &F, start: usize, end: usize)
where
    F: Fn(&T, &T) -> i32,
{
    if start >= end || start >= array.len() {
        return;
    }

    let actual_end = end.min(array.len() - 1);

    for i in (start + 1)..=actual_end {
        let mut j = i;
        while j > start && compare(&array[j - 1], &array[j]) > 0 {
            array.swap(j - 1, j);
            j -= 1;
        }
    }
}
