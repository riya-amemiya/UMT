/// Determines whether a number is prime using 6k +/- 1 trial division.
///
/// # Arguments
///
/// * `n` - Number to check.
///
/// # Returns
///
/// `true` if `n` is a prime integer; `false` for non-integers, NaN, infinities,
/// and numbers below 2.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_is_prime;
///
/// assert!(umt_is_prime(2.0));
/// assert!(umt_is_prime(97.0));
/// assert!(!umt_is_prime(91.0));
/// assert!(!umt_is_prime(7.5));
/// ```
pub fn umt_is_prime(n: f64) -> bool {
    if !n.is_finite() || n.fract() != 0.0 || n < 2.0 {
        return false;
    }
    if n < 4.0 {
        return true;
    }
    if n % 2.0 == 0.0 || n % 3.0 == 0.0 {
        return false;
    }

    let number = n as u64;
    let mut divisor: u64 = 5;
    while divisor * divisor <= number {
        if number.is_multiple_of(divisor) || number.is_multiple_of(divisor + 2) {
            return false;
        }
        divisor += 6;
    }
    true
}
