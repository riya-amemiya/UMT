use super::js_round;

/// Rounds a number to a specified precision.
///
/// This function takes a number and a precision as input and returns the number rounded to the specified precision.
/// Ties round toward positive infinity, so `-2.5` rounds to `-2.0` and `2.5` rounds to `3.0`.
///
/// # Arguments
///
/// * `num` - The number to round.
/// * `precision` - The precision to round to.
///
/// # Returns
///
/// The number rounded to the specified precision.
pub fn umt_round_of(num: f64, precision: i32) -> f64 {
    let factor = 10f64.powi(precision);
    js_round(num * factor) / factor
}
