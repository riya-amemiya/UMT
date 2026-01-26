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
