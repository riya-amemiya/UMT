/// Rounds a number to a specified precision.
///
/// This function takes a number and a precision as input and returns the number rounded to the specified precision.
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
    (num * 10f64.powi(precision)).round() / 10f64.powi(precision)
}
