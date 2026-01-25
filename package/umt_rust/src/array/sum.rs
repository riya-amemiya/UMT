/// Returns the sum of an array of numbers.
///
/// # Arguments
///
/// * `array` - Array of numbers
///
/// # Returns
///
/// Sum of the array elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_sum;
///
/// assert_eq!(umt_sum(&[1.0, 2.0, 3.0]), 6.0);
/// assert_eq!(umt_sum(&[]), 0.0);
/// ```
#[inline]
pub fn umt_sum(array: &[f64]) -> f64 {
    array.iter().sum()
}

/// Returns the sum of an array of integers.
///
/// # Arguments
///
/// * `array` - Array of integers
///
/// # Returns
///
/// Sum of the array elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_sum_i64;
///
/// assert_eq!(umt_sum_i64(&[1, 2, 3]), 6);
/// assert_eq!(umt_sum_i64(&[]), 0);
/// ```
#[inline]
pub fn umt_sum_i64(array: &[i64]) -> i64 {
    array.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_integers() {
        assert_eq!(umt_sum(&[1.0, 2.0, 3.0]), 6.0);
        assert_eq!(umt_sum(&[10.0, 20.0, 30.0]), 60.0);
        assert_eq!(umt_sum(&[-1.0, -2.0, -3.0]), -6.0);
        assert_eq!(umt_sum(&[]), 0.0);
    }

    #[test]
    fn test_sum_decimals() {
        assert!((umt_sum(&[0.1, 0.2, 0.3]) - 0.6).abs() < 1e-10);
        assert!((umt_sum(&[1.1, 2.2, 3.3]) - 6.6).abs() < 1e-10);
    }

    #[test]
    fn test_sum_mixed() {
        assert!((umt_sum(&[1.0, 2.5, 3.7]) - 7.2).abs() < 1e-10);
        assert!((umt_sum(&[-1.5, 2.0, -3.7]) - (-3.2)).abs() < 1e-10);
    }

    #[test]
    fn test_sum_large_numbers() {
        let large_num = 1_000_000_000.0;
        assert_eq!(umt_sum(&[large_num, large_num]), 2.0 * large_num);
        assert_eq!(umt_sum(&[large_num, -large_num]), 0.0);
    }

    #[test]
    fn test_sum_i64() {
        assert_eq!(umt_sum_i64(&[1, 2, 3]), 6);
        assert_eq!(umt_sum_i64(&[10, 20, 30]), 60);
        assert_eq!(umt_sum_i64(&[-1, -2, -3]), -6);
        assert_eq!(umt_sum_i64(&[]), 0);
    }
}
