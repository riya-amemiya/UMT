use crate::math::average::umt_average;
use crate::math::deviation_value::umt_deviation_value;
use crate::math::standard_deviation::umt_standard_deviation;

/// Calculate deviation score (T-score) from an array of values
///
/// # Arguments
/// * `value` - Input value
/// * `average_value` - Array of values to calculate average from
///
/// # Returns
/// Deviation score (50 is average, each standard deviation is worth 10 points)
/// Returns 50 when standard deviation is 0 (all values are the same)
///
/// # Example
/// ```
/// use umt_rust::simple::math::deviation_value_simple::umt_deviation_value_simple_from_array;
/// let scores = vec![40.0, 50.0, 60.0];
/// let score = umt_deviation_value_simple_from_array(60.0, &scores);
/// assert!((score - 62.247).abs() < 0.001);
/// ```
pub fn umt_deviation_value_simple_from_array(value: f64, average_value: &[f64]) -> f64 {
    let avg = umt_average(average_value.to_vec());
    let sd = umt_standard_deviation(average_value);

    if sd == 0.0 {
        50.0
    } else {
        umt_deviation_value(value, avg, sd)
    }
}

/// Calculate deviation score (T-score)
///
/// # Arguments
/// * `value` - Input value
/// * `average_value` - Average value
/// * `standard_deviation_value` - Standard deviation
///
/// # Returns
/// Deviation score (50 is average, each standard deviation is worth 10 points)
/// Returns 50 when standard deviation is 0 (all values are the same)
///
/// # Example
/// ```
/// use umt_rust::simple::math::deviation_value_simple::umt_deviation_value_simple;
/// assert_eq!(umt_deviation_value_simple(60.0, 50.0, 10.0), 60.0);
/// ```
pub fn umt_deviation_value_simple(
    value: f64,
    average_value: f64,
    standard_deviation_value: f64,
) -> f64 {
    if standard_deviation_value == 0.0 {
        50.0
    } else {
        umt_deviation_value(value, average_value, standard_deviation_value)
    }
}
