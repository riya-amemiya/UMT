/// Gets the length of the decimal part of a floating-point number.
///
/// This function takes a floating-point number as input and returns the length of its decimal part.
///
/// # Arguments
///
/// * `value` - The floating-point number.
///
/// # Returns
///
/// The length of the decimal part of the input number.
pub fn umt_get_decimal_length(value: f64) -> usize {
    if value.fract() == 0.0 {
        return 0;
    }
    value
        .to_string()
        .split('.')
        .nth(1)
        .map_or(0, |decimal_part| decimal_part.len())
}
