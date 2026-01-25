use super::umt_get_decimal_length;

/// Performs multiplication without floating point errors for any number of arguments.
///
/// This function handles floating point precision issues by scaling the numbers
/// to integers before performing multiplication, then scaling back.
///
/// # Arguments
///
/// * `numbers` - A slice of f64 numbers to multiply.
///
/// # Returns
///
/// The product of all numbers.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_multiplication;
///
/// let result = umt_multiplication(&[0.1, 0.2, 0.3]);
/// assert!((result - 0.006).abs() < 1e-10);
/// ```
pub fn umt_multiplication(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 1.0;
    }

    numbers.iter().fold(1.0, |accumulator, &number| {
        let n = 10_f64
            .powi((umt_get_decimal_length(accumulator) + umt_get_decimal_length(number)) as i32);
        let acc_str = accumulator.to_string().replace('.', "");
        let num_str = number.to_string().replace('.', "");
        let acc_int: f64 = acc_str.parse().unwrap_or(accumulator);
        let num_int: f64 = num_str.parse().unwrap_or(number);
        (acc_int * num_int) / n
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication_basic() {
        assert_eq!(umt_multiplication(&[2.0, 3.0]), 6.0);
    }

    #[test]
    fn test_multiplication_float() {
        let result = umt_multiplication(&[0.1, 0.2]);
        assert!((result - 0.02).abs() < 1e-10);
    }

    #[test]
    fn test_multiplication_three_floats() {
        let result = umt_multiplication(&[0.1, 0.2, 0.3]);
        assert!((result - 0.006).abs() < 1e-10);
    }

    #[test]
    fn test_multiplication_empty() {
        assert_eq!(umt_multiplication(&[]), 1.0);
    }

    #[test]
    fn test_multiplication_single() {
        assert_eq!(umt_multiplication(&[5.0]), 5.0);
    }

    #[test]
    fn test_multiplication_with_zero() {
        assert_eq!(umt_multiplication(&[5.0, 0.0]), 0.0);
    }

    #[test]
    fn test_multiplication_negative() {
        assert_eq!(umt_multiplication(&[-2.0, 3.0]), -6.0);
    }
}
