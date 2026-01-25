use crate::math::{umt_average, umt_deviation_value};

/// Calculates the standard deviation of a population.
///
/// # Arguments
///
/// * `values` - A slice of f64 numbers.
///
/// # Returns
///
/// The population standard deviation of the values.
fn standard_deviation(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let avg = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter().map(|x| (x - avg).powi(2)).sum::<f64>() / values.len() as f64;
    variance.sqrt()
}

/// Calculates the deviation value (T-score) using an array of reference values.
///
/// This function computes the deviation value by first calculating the mean and
/// standard deviation from the provided array, then applying the deviation value formula.
///
/// # Arguments
///
/// * `value` - The input value to calculate deviation for.
/// * `reference_values` - A vector of f64 values to calculate average and standard deviation from.
///
/// # Returns
///
/// The deviation score (50 is average, each standard deviation is worth 10 points).
/// Returns 50 when standard deviation is 0 (all values are the same).
///
/// # Examples
///
/// ```
/// use umt_rust::simple::math::umt_deviation_value_simple_from_array;
///
/// let scores = vec![40.0, 50.0, 60.0];
/// let result = umt_deviation_value_simple_from_array(60.0, scores);
/// assert!((result - 62.25).abs() < 0.01);
/// ```
#[inline]
pub fn umt_deviation_value_simple_from_array(value: f64, reference_values: Vec<f64>) -> f64 {
    let avg = umt_average(reference_values.clone());
    let sd = standard_deviation(&reference_values);

    if sd == 0.0 {
        50.0
    } else {
        umt_deviation_value(value, avg, sd)
    }
}

/// Calculates the deviation value (T-score) using explicit average and standard deviation.
///
/// # Arguments
///
/// * `value` - The input value to calculate deviation for.
/// * `average_value` - The average value.
/// * `standard_deviation_value` - The standard deviation value.
///
/// # Returns
///
/// The deviation score (50 is average, each standard deviation is worth 10 points).
/// Returns 50 when standard deviation is 0.
///
/// # Examples
///
/// ```
/// use umt_rust::simple::math::umt_deviation_value_simple;
///
/// // When value equals average (no deviation)
/// assert_eq!(umt_deviation_value_simple(50.0, 50.0, 10.0), 50.0);
///
/// // One standard deviation above average
/// assert_eq!(umt_deviation_value_simple(60.0, 50.0, 10.0), 60.0);
///
/// // When standard deviation is 0, returns 50
/// assert_eq!(umt_deviation_value_simple(100.0, 50.0, 0.0), 50.0);
/// ```
#[inline]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deviation_value_simple_with_explicit_values() {
        // When value equals average (no deviation)
        assert_eq!(umt_deviation_value_simple(50.0, 50.0, 10.0), 50.0);

        // One standard deviation above average
        assert_eq!(umt_deviation_value_simple(60.0, 50.0, 10.0), 60.0);

        // One standard deviation below average
        assert_eq!(umt_deviation_value_simple(40.0, 50.0, 10.0), 40.0);

        // When standard deviation is 0
        assert_eq!(umt_deviation_value_simple(100.0, 50.0, 0.0), 50.0);
    }

    #[test]
    fn test_deviation_value_simple_with_array() {
        // Using simple array [40, 50, 60]
        // mean = 50
        // population standard deviation ~= 8.165
        let scores = vec![40.0, 50.0, 60.0];

        let result = umt_deviation_value_simple_from_array(60.0, scores.clone());
        assert!((result - 62.25).abs() < 0.01); // +1.225 SD

        let result = umt_deviation_value_simple_from_array(50.0, scores.clone());
        assert_eq!(result, 50.0); // mean

        let result = umt_deviation_value_simple_from_array(40.0, scores);
        assert!((result - 37.75).abs() < 0.01); // -1.225 SD
    }

    #[test]
    fn test_deviation_value_simple_all_same_values() {
        let same_scores = vec![50.0, 50.0, 50.0];
        assert_eq!(
            umt_deviation_value_simple_from_array(50.0, same_scores.clone()),
            50.0
        );
        assert_eq!(
            umt_deviation_value_simple_from_array(0.0, same_scores.clone()),
            50.0
        );
        assert_eq!(
            umt_deviation_value_simple_from_array(100.0, same_scores),
            50.0
        );
    }
}
