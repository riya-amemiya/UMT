use super::umt_gcd;

/// Result of reducing a fraction.
#[derive(Debug, Clone, PartialEq)]
pub struct ReduceResult {
    pub x: i32,
    pub y: i32,
    pub gcd: i32,
}

/// Reduces a fraction to its lowest terms.
///
/// # Arguments
///
/// * `x` - Numerator.
/// * `y` - Denominator.
///
/// # Returns
///
/// A ReduceResult containing the reduced numerator, denominator, and the GCD used for reduction.
/// Returns None if the denominator is zero.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_reduce;
///
/// let result = umt_reduce(2, 4).unwrap();
/// assert_eq!(result.x, 1);
/// assert_eq!(result.y, 2);
/// assert_eq!(result.gcd, 2);
/// ```
pub fn umt_reduce(x: i32, y: i32) -> Option<ReduceResult> {
    if y == 0 {
        return None;
    }

    if x == 0 {
        return Some(ReduceResult {
            x: 0,
            y: 1,
            gcd: y.abs(),
        });
    }

    let gcd_value = umt_gcd(x.abs(), y.abs());
    let sign = if y < 0 { -1 } else { 1 };

    Some(ReduceResult {
        x: (x / gcd_value) * sign,
        y: (y / gcd_value).abs(),
        gcd: gcd_value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_basic() {
        let result = umt_reduce(2, 4).unwrap();
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 2);
        assert_eq!(result.gcd, 2);
    }

    #[test]
    fn test_reduce_already_reduced() {
        let result = umt_reduce(1, 3).unwrap();
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 3);
        assert_eq!(result.gcd, 1);
    }

    #[test]
    fn test_reduce_zero_numerator() {
        let result = umt_reduce(0, 5).unwrap();
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 1);
        assert_eq!(result.gcd, 5);
    }

    #[test]
    fn test_reduce_zero_denominator() {
        assert!(umt_reduce(5, 0).is_none());
    }

    #[test]
    fn test_reduce_negative_denominator() {
        let result = umt_reduce(2, -4).unwrap();
        assert_eq!(result.x, -1);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn test_reduce_negative_numerator() {
        let result = umt_reduce(-2, 4).unwrap();
        assert_eq!(result.x, -1);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn test_reduce_both_negative() {
        let result = umt_reduce(-2, -4).unwrap();
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 2);
    }
}
