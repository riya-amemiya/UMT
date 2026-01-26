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
