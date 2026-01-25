/// Generates an array of multiples of a number.
///
/// # Arguments
///
/// * `x` - Base number.
/// * `n` - Number of multiples to generate.
///
/// # Returns
///
/// A vector of n multiples of x.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_multiples;
///
/// assert_eq!(umt_multiples(2.0, 5), vec![2.0, 4.0, 6.0, 8.0, 10.0]);
/// ```
pub fn umt_multiples(x: f64, n: usize) -> Vec<f64> {
    (1..=n).map(|i| x * i as f64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiples_basic() {
        assert_eq!(umt_multiples(2.0, 5), vec![2.0, 4.0, 6.0, 8.0, 10.0]);
    }

    #[test]
    fn test_multiples_one() {
        assert_eq!(umt_multiples(3.0, 1), vec![3.0]);
    }

    #[test]
    fn test_multiples_zero() {
        let result: Vec<f64> = vec![];
        assert_eq!(umt_multiples(5.0, 0), result);
    }

    #[test]
    fn test_multiples_float() {
        let result = umt_multiples(0.5, 4);
        assert_eq!(result, vec![0.5, 1.0, 1.5, 2.0]);
    }

    #[test]
    fn test_multiples_negative() {
        assert_eq!(umt_multiples(-2.0, 3), vec![-2.0, -4.0, -6.0]);
    }
}
