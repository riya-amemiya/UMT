/// Default comparison function that returns:
/// - 1 if a > b
/// - -1 if a < b
/// - 0 if a equals b
///
/// # Arguments
///
/// * `a` - First value to compare
/// * `b` - Second value to compare
///
/// # Returns
///
/// Comparison result (-1, 0, or 1)
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_compare_function_default;
///
/// assert_eq!(umt_compare_function_default(&2, &1), 1);
/// assert_eq!(umt_compare_function_default(&1, &2), -1);
/// assert_eq!(umt_compare_function_default(&2, &2), 0);
/// ```
#[inline]
pub fn umt_compare_function_default<T: PartialOrd>(a: &T, b: &T) -> i32 {
    if a > b {
        1
    } else if a < b {
        -1
    } else {
        0
    }
}

/// Type alias for comparison functions
pub type CompareFunction<T> = fn(&T, &T) -> i32;
