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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_basic() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(umt_merge_sort(&arr, None), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_merge_sort_empty() {
        let arr: Vec<i32> = vec![];
        assert_eq!(umt_merge_sort(&arr, None), Vec::<i32>::new());
    }

    #[test]
    fn test_merge_sort_single() {
        let arr = vec![42];
        assert_eq!(umt_merge_sort(&arr, None), vec![42]);
    }

    #[test]
    fn test_merge_sort_already_sorted() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(umt_merge_sort(&arr, None), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_reverse() {
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(umt_merge_sort(&arr, None), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_with_custom_compare() {
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
        assert_eq!(umt_merge_sort(&arr, Some(descending)), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_merge_sort_strings() {
        let arr = vec!["banana", "apple", "cherry", "date"];
        assert_eq!(
            umt_merge_sort(&arr, None),
            vec!["apple", "banana", "cherry", "date"]
        );
    }

    #[test]
    fn test_merge_sort_does_not_mutate() {
        let arr = vec![3, 1, 4, 1, 5];
        let _ = umt_merge_sort(&arr, None);
        assert_eq!(arr, vec![3, 1, 4, 1, 5]);
    }

    #[test]
    fn test_merge_sort_duplicates() {
        let arr = vec![3, 3, 1, 1, 2, 2];
        assert_eq!(umt_merge_sort(&arr, None), vec![1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn test_merge_sort_large() {
        let arr: Vec<i32> = (0..1000).rev().collect();
        let sorted = umt_merge_sort(&arr, None);
        assert_eq!(sorted, (0..1000).collect::<Vec<i32>>());
    }

    #[test]
    fn test_merge_sort_floats() {
        let arr = vec![3.14, 2.71, 1.41, 1.73];
        let sorted = umt_merge_sort(&arr, None);
        assert_eq!(sorted, vec![1.41, 1.73, 2.71, 3.14]);
    }
}
