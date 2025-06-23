/// Calculates the deviation value.
///
/// This function takes a value, an average value, and a standard deviation value as input and returns the deviation value.
///
/// # Arguments
///
/// * `value` - The value.
/// * `average_value` - The average value.
/// * `standard_deviation_value` - The standard deviation value.
///
/// # Returns
///
/// The deviation value.
pub fn umt_deviation_value(value: f64, average_value: f64, standard_deviation_value: f64) -> f64 {
    (value - average_value) / standard_deviation_value * 10.0 + 50.0
}
