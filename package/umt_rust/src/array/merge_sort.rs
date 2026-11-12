use super::compare_function_default::umt_compare_function_default;

/// Merge sort implementation.
///
/// # Arguments
///
/// * `array` - Array to sort
/// * `compare` - Optional comparison function
///
/// # Returns
///
/// Sorted array
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_merge_sort;
///
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// assert_eq!(umt_merge_sort(&arr, None), vec![1, 1, 2, 3, 4, 5, 6, 9]);
/// ```
pub fn umt_merge_sort<T: Clone + PartialOrd>(
    array: &[T],
    compare: Option<fn(&T, &T) -> i32>,
) -> Vec<T> {
    if array.len() <= 1 {
        return array.to_vec();
    }

    let compare_fn = compare.unwrap_or(umt_compare_function_default);
    let mut result = array.to_vec();
    let mut aux = array.to_vec();
    merge_sort_impl(&mut result, &mut aux, 0, array.len(), &compare_fn);
    result
}

fn merge_sort_impl<T: Clone, F>(
    array: &mut [T],
    aux: &mut [T],
    start: usize,
    end: usize,
    compare: &F,
) where
    F: Fn(&T, &T) -> i32,
{
    if end - start <= 1 {
        return;
    }

    let mid = start + (end - start) / 2;
    merge_sort_impl(array, aux, start, mid, compare);
    merge_sort_impl(array, aux, mid, end, compare);

    merge(array, aux, start, mid, end, compare);
}

fn merge<T: Clone, F>(
    array: &mut [T],
    aux: &mut [T],
    start: usize,
    mid: usize,
    end: usize,
    compare: &F,
) where
    F: Fn(&T, &T) -> i32,
{
    aux[start..end].clone_from_slice(&array[start..end]);

    let mut left_idx = start;
    let mut right_idx = mid;

    for slot in &mut array[start..end] {
        if left_idx < mid && (right_idx >= end || compare(&aux[left_idx], &aux[right_idx]) <= 0) {
            *slot = aux[left_idx].clone();
            left_idx += 1;
        } else {
            *slot = aux[right_idx].clone();
            right_idx += 1;
        }
    }
}
