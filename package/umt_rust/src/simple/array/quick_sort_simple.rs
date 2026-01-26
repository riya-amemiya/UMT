use std::cmp::Ordering;

/// Default comparison function that returns:
/// - `Ordering::Greater` if a > b
/// - `Ordering::Less` if a < b
/// - `Ordering::Equal` if a equals b
///
/// # Arguments
///
/// * `a` - First value to compare
/// * `b` - Second value to compare
///
/// # Returns
///
/// Comparison result as `Ordering`
#[inline]
pub fn compare_function_default<T: PartialOrd>(a: &T, b: &T) -> Ordering {
    a.partial_cmp(b).unwrap_or(Ordering::Equal)
}

/// Finds the median value among three elements in the slice
fn median_of_three<T, F>(array: &[T], a: usize, b: usize, c: usize, compare_function: &F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    let ab = compare_function(&array[a], &array[b]);
    if ab == Ordering::Less {
        let bc = compare_function(&array[b], &array[c]);
        if bc == Ordering::Less {
            b
        } else if compare_function(&array[a], &array[c]) == Ordering::Less {
            c
        } else {
            a
        }
    } else {
        let ac = compare_function(&array[a], &array[c]);
        if ac == Ordering::Less {
            a
        } else if compare_function(&array[b], &array[c]) == Ordering::Less {
            c
        } else {
            b
        }
    }
}

/// Insertion sort for small subarrays
fn insertion_sort<T, F>(array: &mut [T], low: usize, high: usize, compare_function: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    for i in (low + 1)..=high {
        let mut j = i;
        while j > low && compare_function(&array[j - 1], &array[j]) == Ordering::Greater {
            array.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Partitions the array around a pivot element using median-of-three strategy
fn partition<T, F>(array: &mut [T], low: usize, high: usize, compare_function: &F) -> usize
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let mid = low + (high - low) / 2;
    let pivot_idx = median_of_three(array, low, mid, high, compare_function);

    // Move pivot to end
    array.swap(pivot_idx, high);

    let mut store_index = low;
    for i in low..high {
        if compare_function(&array[i], &array[high]) == Ordering::Less {
            array.swap(i, store_index);
            store_index += 1;
        }
    }
    array.swap(store_index, high);
    store_index
}

/// Internal implementation of the quicksort algorithm
fn sort_impl<T, F>(
    array: &mut [T],
    low: usize,
    high: usize,
    compare_function: &F,
    insertion_sort_threshold: usize,
) where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    if low >= high {
        return;
    }

    // Use insertion sort for small subarrays
    if high - low < insertion_sort_threshold {
        insertion_sort(array, low, high, compare_function);
        return;
    }

    let pivot_index = partition(array, low, high, compare_function);

    // Recursively sort the smaller partition first to limit stack depth
    if pivot_index > 0 && pivot_index - low < high - pivot_index {
        if pivot_index > 0 {
            sort_impl(
                array,
                low,
                pivot_index - 1,
                compare_function,
                insertion_sort_threshold,
            );
        }
        sort_impl(
            array,
            pivot_index + 1,
            high,
            compare_function,
            insertion_sort_threshold,
        );
    } else {
        sort_impl(
            array,
            pivot_index + 1,
            high,
            compare_function,
            insertion_sort_threshold,
        );
        if pivot_index > 0 {
            sort_impl(
                array,
                low,
                pivot_index - 1,
                compare_function,
                insertion_sort_threshold,
            );
        }
    }
}

/// Quick sort implementation for arrays with automatic range validation.
///
/// This is a simplified version of quick sort that automatically validates
/// and corrects the start and end indices if they are out of bounds.
///
/// # Arguments
///
/// * `array` - Input slice to sort
/// * `compare_function` - Optional comparison function (defaults to natural ordering)
/// * `start_id` - Starting index for sorting (will be corrected if invalid)
/// * `end_id` - Ending index for sorting (will be corrected if invalid)
///
/// # Returns
///
/// A new sorted vector
///
/// # Examples
///
/// ```
/// use umt_rust::simple::array::umt_quick_sort_simple;
/// use std::cmp::Ordering;
///
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
/// let sorted = umt_quick_sort_simple::<i32, fn(&i32, &i32) -> Ordering>(&arr, None, None, None);
/// assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 6, 9]);
/// ```
///
/// ```
/// use umt_rust::simple::array::umt_quick_sort_simple;
/// use std::cmp::Ordering;
///
/// let arr = vec![3, 1, 4, 1, 5];
/// // Sort in descending order
/// let sorted = umt_quick_sort_simple(&arr, Some(|a: &i32, b: &i32| b.cmp(a)), None, None);
/// assert_eq!(sorted, vec![5, 4, 3, 1, 1]);
/// ```
pub fn umt_quick_sort_simple<T, F>(
    array: &[T],
    compare_function: Option<F>,
    start_id: Option<isize>,
    end_id: Option<isize>,
) -> Vec<T>
where
    T: Clone + PartialOrd,
    F: Fn(&T, &T) -> Ordering,
{
    if array.is_empty() {
        return Vec::new();
    }

    let len = array.len() as isize;
    let mut local_start_id = start_id.unwrap_or(0);
    let mut local_end_id = end_id.unwrap_or(len - 1);

    // Validate and correct start_id
    if local_start_id < 0 || local_start_id >= len || local_start_id > local_end_id {
        local_start_id = 0;
    }

    // Validate and correct end_id
    if local_end_id < 0 || local_end_id >= len {
        local_end_id = len - 1;
    }

    let start = local_start_id as usize;
    let end = local_end_id as usize;

    let mut result = array.to_vec();

    if start >= end {
        return result;
    }

    match compare_function {
        Some(cmp) => {
            sort_impl(&mut result, start, end, &cmp, 10);
        }
        None => {
            sort_impl(&mut result, start, end, &compare_function_default, 10);
        }
    }

    result
}

/// Quick sort implementation for f64 arrays with automatic range validation.
///
/// This is a convenience function for sorting f64 arrays.
///
/// # Arguments
///
/// * `array` - Input slice of f64 to sort
/// * `ascending` - Sort in ascending order if true, descending if false
/// * `start_id` - Starting index for sorting (will be corrected if invalid)
/// * `end_id` - Ending index for sorting (will be corrected if invalid)
///
/// # Returns
///
/// A new sorted vector
///
/// # Examples
///
/// ```
/// use umt_rust::simple::array::umt_quick_sort_simple_f64;
///
/// let arr = vec![3.0, 1.0, 4.0, 1.0, 5.0];
/// let sorted = umt_quick_sort_simple_f64(&arr, true, None, None);
/// assert_eq!(sorted, vec![1.0, 1.0, 3.0, 4.0, 5.0]);
/// ```
#[inline]
pub fn umt_quick_sort_simple_f64(
    array: &[f64],
    ascending: bool,
    start_id: Option<isize>,
    end_id: Option<isize>,
) -> Vec<f64> {
    if ascending {
        umt_quick_sort_simple(
            array,
            Some(|a: &f64, b: &f64| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
            start_id,
            end_id,
        )
    } else {
        umt_quick_sort_simple(
            array,
            Some(|a: &f64, b: &f64| b.partial_cmp(a).unwrap_or(Ordering::Equal)),
            start_id,
            end_id,
        )
    }
}

/// Quick sort implementation for i32 arrays with automatic range validation.
///
/// This is a convenience function for sorting i32 arrays.
///
/// # Arguments
///
/// * `array` - Input slice of i32 to sort
/// * `ascending` - Sort in ascending order if true, descending if false
/// * `start_id` - Starting index for sorting (will be corrected if invalid)
/// * `end_id` - Ending index for sorting (will be corrected if invalid)
///
/// # Returns
///
/// A new sorted vector
///
/// # Examples
///
/// ```
/// use umt_rust::simple::array::umt_quick_sort_simple_i32;
///
/// let arr = vec![3, 1, 4, 1, 5];
/// let sorted = umt_quick_sort_simple_i32(&arr, true, None, None);
/// assert_eq!(sorted, vec![1, 1, 3, 4, 5]);
/// ```
#[inline]
pub fn umt_quick_sort_simple_i32(
    array: &[i32],
    ascending: bool,
    start_id: Option<isize>,
    end_id: Option<isize>,
) -> Vec<i32> {
    if ascending {
        umt_quick_sort_simple(array, Some(|a: &i32, b: &i32| a.cmp(b)), start_id, end_id)
    } else {
        umt_quick_sort_simple(array, Some(|a: &i32, b: &i32| b.cmp(a)), start_id, end_id)
    }
}
