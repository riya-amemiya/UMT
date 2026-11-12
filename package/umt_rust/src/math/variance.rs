use super::{umt_addition, umt_average, umt_division};

/// Calculates the variance of a set of values.
///
/// # Arguments
///
/// * `values` - A slice of f64 values.
/// * `sample` - If `true`, divides by `n - 1` (sample variance) instead of `n`
///   (population variance).
///
/// # Returns
///
/// The variance, 0.0 for an empty slice, or NaN for a sample of a single value.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_variance;
///
/// assert!((umt_variance(&[1.0, 2.0, 3.0], false) - 0.6666666666666666).abs() < 1e-10);
/// assert_eq!(umt_variance(&[1.0, 2.0, 3.0], true), 1.0);
/// assert_eq!(umt_variance(&[10.0, 12.0, 23.0, 23.0, 16.0, 23.0, 21.0, 16.0], false), 24.0);
/// ```
pub fn umt_variance(values: &[f64], sample: bool) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    if sample && values.len() == 1 {
        return f64::NAN;
    }

    let avg = umt_average(values.to_vec());
    let square_diffs: Vec<f64> = values
        .iter()
        .map(|&value| {
            let diff = value - avg;
            diff * diff
        })
        .collect();

    let divisor = if sample {
        values.len() - 1
    } else {
        values.len()
    };
    umt_division(umt_addition(&square_diffs), divisor as f64)
}
