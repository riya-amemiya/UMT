use super::{umt_get_decimal_length, umt_max, umt_multiplication};

/// Performs addition without floating point errors.
///
/// This function handles floating point precision issues by scaling the numbers
/// to integers before performing addition, then scaling back.
///
/// # Arguments
///
/// * `numbers` - A slice of f64 numbers to add.
///
/// # Returns
///
/// The sum of all numbers.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_addition;
///
/// let result = umt_addition(&[0.1, 0.2]);
/// assert!((result - 0.3).abs() < 1e-10);
/// ```
pub fn umt_addition(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }

    let decimal_lengths: Vec<f64> = numbers
        .iter()
        .map(|&n| umt_get_decimal_length(n) as f64)
        .collect();
    let z = 10_f64.powi(umt_max(&decimal_lengths) as i32);

    let sum: f64 = numbers
        .iter()
        .map(|&n| umt_multiplication(&[n, z]))
        .sum();

    sum / z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition_integers() {
        assert_eq!(umt_addition(&[2.0, 3.0]), 5.0);
    }

    #[test]
    fn test_addition_negative() {
        assert_eq!(umt_addition(&[-2.0, -3.0]), -5.0);
    }

    #[test]
    fn test_addition_mixed() {
        assert_eq!(umt_addition(&[2.0, -3.0]), -1.0);
    }

    #[test]
    fn test_addition_float() {
        let result = umt_addition(&[0.1, 0.2]);
        assert!((result - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_addition_negative_float() {
        let result = umt_addition(&[-0.1, -0.2]);
        assert!((result - (-0.3)).abs() < 1e-10);
    }

    #[test]
    fn test_addition_three_numbers() {
        assert_eq!(umt_addition(&[1.0, 2.0, 3.0]), 6.0);
    }

    #[test]
    fn test_addition_three_floats() {
        let result = umt_addition(&[0.1, 0.2, 0.3]);
        assert!((result - 0.6).abs() < 1e-10);
    }

    #[test]
    fn test_addition_empty() {
        assert_eq!(umt_addition(&[]), 0.0);
    }

    #[test]
    fn test_addition_single() {
        assert_eq!(umt_addition(&[5.0]), 5.0);
    }
}
