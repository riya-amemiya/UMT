use super::umt_get_decimal_length;

/// Result of division with quotient and remainder.
#[derive(Debug, Clone, PartialEq)]
pub struct DivisionResult {
    pub quotient: i64,
    pub remainder: i64,
}

/// Performs division without floating point errors.
///
/// This function handles floating point precision issues by converting to integers.
///
/// # Arguments
///
/// * `x` - Dividend.
/// * `y` - Divisor.
///
/// # Returns
///
/// The division result as f64, or NaN if dividing by zero.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_division;
///
/// let result = umt_division(0.1, 0.2);
/// assert!((result - 0.5).abs() < 1e-10);
/// ```
pub fn umt_division(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        return f64::NAN;
    }

    let sign = x.signum() * y.signum();
    let abs_x = x.abs();
    let abs_y = y.abs();

    // Get decimal lengths without swapping
    let decimal_length_x = umt_get_decimal_length(abs_x);
    let decimal_length_y = umt_get_decimal_length(abs_y);

    // Remove decimal points
    let x_int: f64 = abs_x.to_string().replace('.', "").parse().unwrap_or(abs_x);
    let y_int: f64 = abs_y.to_string().replace('.', "").parse().unwrap_or(abs_y);

    // Calculate scaling factor based on the actual decimal difference
    let scaling_factor = 10_f64.powi((decimal_length_y as i32) - (decimal_length_x as i32));

    // Calculate the result
    let division_result = (x_int / y_int) * scaling_factor;

    sign * division_result
}

/// Performs division and returns quotient and remainder.
///
/// # Arguments
///
/// * `x` - Dividend.
/// * `y` - Divisor.
///
/// # Returns
///
/// A DivisionResult with quotient and remainder, or None if dividing by zero.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_division_with_remainder;
///
/// let result = umt_division_with_remainder(10.0, 3.0).unwrap();
/// assert_eq!(result.quotient, 3);
/// assert_eq!(result.remainder, 1);
/// ```
pub fn umt_division_with_remainder(x: f64, y: f64) -> Option<DivisionResult> {
    if y == 0.0 {
        return None;
    }

    let sign = (x.signum() * y.signum()) as i64;
    let abs_x = x.abs();
    let abs_y = y.abs();

    // Get decimal lengths
    let decimal_length_x = umt_get_decimal_length(abs_x);
    let decimal_length_y = umt_get_decimal_length(abs_y);

    // Remove decimal points
    let x_int: i64 = abs_x
        .to_string()
        .replace('.', "")
        .parse()
        .unwrap_or(abs_x as i64);
    let y_int: i64 = abs_y
        .to_string()
        .replace('.', "")
        .parse()
        .unwrap_or(abs_y as i64);

    // Calculate scaling factor based on the actual decimal difference
    let scaling_factor = 10_f64.powi((decimal_length_y as i32) - (decimal_length_x as i32));

    // Calculate the result
    let division_result = ((x_int as f64 / y_int as f64) * scaling_factor).floor() as i64;
    let remainder = x_int % y_int;

    Some(DivisionResult {
        quotient: sign * division_result,
        remainder,
    })
}
