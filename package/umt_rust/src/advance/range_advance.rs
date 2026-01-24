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
        None => umt_range(start, end),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_advance_returns_full_range_without_condition() {
        let result = umt_range_advance(0, 5, None::<fn(i32) -> bool>);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_range_advance_with_start_and_end() {
        let result = umt_range_advance(2, 5, None::<fn(i32) -> bool>);
        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn test_range_advance_with_even_condition() {
        let is_even = |num: i32| num % 2 == 0;
        let result = umt_range_advance(0, 10, Some(is_even));
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_range_advance_with_odd_condition() {
        let is_odd = |num: i32| num % 2 != 0;
        let result = umt_range_advance(0, 10, Some(is_odd));
        assert_eq!(result, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_range_advance_empty_when_start_greater_than_end() {
        let result = umt_range_advance(5, 2, None::<fn(i32) -> bool>);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_range_advance_empty_when_start_equals_end_with_unsatisfied_condition() {
        let is_odd = |num: i32| num % 2 != 0;
        let result = umt_range_advance(5, 5, Some(is_odd));
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_range_advance_with_negative_numbers() {
        let is_negative = |num: i32| num < 0;
        let result = umt_range_advance(-5, 5, Some(is_negative));
        assert_eq!(result, vec![-5, -4, -3, -2, -1]);
    }

    #[test]
    fn test_range_advance_filtered_with_even() {
        let result = umt_range_advance_filtered(0, 10, |n| n % 2 == 0);
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_range_advance_filtered_divisible_by_three() {
        let result = umt_range_advance_filtered(0, 15, |n| n % 3 == 0);
        assert_eq!(result, vec![0, 3, 6, 9, 12]);
    }
}
