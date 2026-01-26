use crate::array::umt_range;

/// Returns a vector of numbers that satisfy the conditional expression.
///
/// This function generates a range of numbers from `start` to `end` (exclusive)
/// and optionally filters them using a provided conditional expression.
///
/// # Arguments
///
/// * `start` - The starting number (inclusive).
/// * `end` - The ending number (exclusive).
/// * `conditional_expression` - Optional function that determines which numbers to include.
///
/// # Returns
///
/// A vector of numbers that satisfy the conditional expression, or all numbers
/// in the range if no conditional expression is provided.
///
/// # Examples
///
/// ```
/// use umt_rust::advance::umt_range_advance;
///
/// // Without conditional expression - returns full range
/// let result = umt_range_advance(1, 5, None::<fn(i32) -> bool>);
/// assert_eq!(result, vec![1, 2, 3, 4]);
///
/// // With conditional expression - returns only even numbers
/// let result = umt_range_advance(1, 10, Some(|n: i32| n % 2 == 0));
/// assert_eq!(result, vec![2, 4, 6, 8]);
/// ```
#[inline]
pub fn umt_range_advance<F>(start: i32, end: i32, conditional_expression: Option<F>) -> Vec<i32>
where
    F: Fn(i32) -> bool,
{
    match conditional_expression {
        Some(condition) => {
            let mut result: Vec<i32> = Vec::new();
            for index in start..end {
                if condition(index) {
                    result.push(index);
                }
            }
            result
        }
        None => umt_range(start, Some(end), None),
    }
}

/// Returns a vector of numbers that satisfy the conditional expression.
///
/// This is a convenience function that always takes a filter function.
/// Use this when you always want to filter the range.
///
/// # Arguments
///
/// * `start` - The starting number (inclusive).
/// * `end` - The ending number (exclusive).
/// * `conditional_expression` - Function that determines which numbers to include.
///
/// # Returns
///
/// A vector of numbers that satisfy the conditional expression.
///
/// # Examples
///
/// ```
/// use umt_rust::advance::umt_range_advance_filtered;
///
/// // Returns only odd numbers
/// let result = umt_range_advance_filtered(0, 10, |n| n % 2 != 0);
/// assert_eq!(result, vec![1, 3, 5, 7, 9]);
/// ```
#[inline]
pub fn umt_range_advance_filtered<F>(start: i32, end: i32, conditional_expression: F) -> Vec<i32>
where
    F: Fn(i32) -> bool,
{
    let mut result: Vec<i32> = Vec::new();
    for index in start..end {
        if conditional_expression(index) {
            result.push(index);
        }
    }
    result
}
