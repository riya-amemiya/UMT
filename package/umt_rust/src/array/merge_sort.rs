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
    merge_sort_impl(array, &compare_fn)
}

fn merge_sort_impl<T: Clone, F>(array: &[T], compare: &F) -> Vec<T>
where
    F: Fn(&T, &T) -> i32,
{
    if array.len() <= 1 {
        return array.to_vec();
    }

    let middle = array.len() / 2;
    let left = merge_sort_impl(&array[..middle], compare);
    let right = merge_sort_impl(&array[middle..], compare);

    merge(&left, &right, compare)
}

fn merge<T: Clone, F>(left: &[T], right: &[T], compare: &F) -> Vec<T>
where
    F: Fn(&T, &T) -> i32,
{
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_idx = 0;
    let mut right_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        if compare(&left[left_idx], &right[right_idx]) <= 0 {
            result.push(left[left_idx].clone());
            left_idx += 1;
        } else {
            result.push(right[right_idx].clone());
            right_idx += 1;
        }
    }

    result.extend_from_slice(&left[left_idx..]);
    result.extend_from_slice(&right[right_idx..]);

    result
}
