use super::umt_average;

/// Calculates the Pearson correlation coefficient between two arrays.
///
/// The correlation coefficient measures the linear relationship between two variables.
/// Values range from -1 (perfect negative correlation) to 1 (perfect positive correlation).
///
/// # Arguments
///
/// * `x` - First array of numbers.
/// * `y` - Second array of numbers.
///
/// # Returns
///
/// Correlation coefficient (-1 to 1), or NaN for invalid inputs.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_correlation_coefficient;
///
/// // Perfect positive correlation
/// let r = umt_correlation_coefficient(&[1.0, 2.0, 3.0, 4.0, 5.0], &[2.0, 4.0, 6.0, 8.0, 10.0]);
/// assert!((r - 1.0).abs() < 1e-10);
///
/// // Perfect negative correlation
/// let r = umt_correlation_coefficient(&[1.0, 2.0, 3.0, 4.0, 5.0], &[5.0, 4.0, 3.0, 2.0, 1.0]);
/// assert!((r - (-1.0)).abs() < 1e-10);
/// ```
pub fn umt_correlation_coefficient(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() {
        return f64::NAN;
    }

    if x.is_empty() {
        return f64::NAN;
    }

    if x.len() == 1 {
        return f64::NAN;
    }

    let mean_x = umt_average(x.to_vec());
    let mean_y = umt_average(y.to_vec());

    let mut numerator = 0.0;
    let mut sum_squared_x = 0.0;
    let mut sum_squared_y = 0.0;

    for (i, &elem) in x.iter().enumerate() {
        let delta_x = elem - mean_x;
        let delta_y = y[i] - mean_y;

        numerator += delta_x * delta_y;
        sum_squared_x += delta_x * delta_x;
        sum_squared_y += delta_y * delta_y;
    }

    let denominator = (sum_squared_x * sum_squared_y).sqrt();

    if denominator == 0.0 {
        return f64::NAN;
    }

    numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_positive_correlation() {
        let r = umt_correlation_coefficient(
            &[1.0, 2.0, 3.0, 4.0, 5.0],
            &[2.0, 4.0, 6.0, 8.0, 10.0],
        );
        assert!((r - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_perfect_negative_correlation() {
        let r = umt_correlation_coefficient(
            &[1.0, 2.0, 3.0, 4.0, 5.0],
            &[5.0, 4.0, 3.0, 2.0, 1.0],
        );
        assert!((r - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_no_correlation() {
        let r = umt_correlation_coefficient(
            &[1.0, 2.0, 3.0, 4.0, 5.0],
            &[1.0, 1.0, 1.0, 1.0, 1.0],
        );
        assert!(r.is_nan());
    }

    #[test]
    fn test_different_lengths() {
        let r = umt_correlation_coefficient(&[1.0, 2.0, 3.0], &[1.0, 2.0]);
        assert!(r.is_nan());
    }

    #[test]
    fn test_empty_arrays() {
        let r = umt_correlation_coefficient(&[], &[]);
        assert!(r.is_nan());
    }

    #[test]
    fn test_single_element() {
        let r = umt_correlation_coefficient(&[1.0], &[2.0]);
        assert!(r.is_nan());
    }
}
