/// Computes quotient and remainder of division.
///
/// # Arguments
///
/// * `x` - Dividend.
/// * `y` - Divisor.
///
/// # Returns
///
/// A tuple containing (quotient, remainder).
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_quotient;
///
/// let (q, r) = umt_quotient(5.0, 2.0);
/// assert_eq!(q, 2.0);
/// assert_eq!(r, 1.0);
/// ```
#[inline]
pub fn umt_quotient(x: f64, y: f64) -> (f64, f64) {
    let remainder = x % y;
    let quotient = (x - remainder) / y;
    (quotient, remainder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quotient_basic() {
        let (q, r) = umt_quotient(5.0, 2.0);
        assert_eq!(q, 2.0);
        assert_eq!(r, 1.0);
    }

    #[test]
    fn test_quotient_exact() {
        let (q, r) = umt_quotient(6.0, 2.0);
        assert_eq!(q, 3.0);
        assert_eq!(r, 0.0);
    }

    #[test]
    fn test_quotient_negative() {
        let (q, r) = umt_quotient(-5.0, 2.0);
        assert_eq!(q, -2.0);
        assert_eq!(r, -1.0);
    }

    #[test]
    fn test_quotient_larger() {
        let (q, r) = umt_quotient(10.0, 3.0);
        assert_eq!(q, 3.0);
        assert_eq!(r, 1.0);
    }
}
