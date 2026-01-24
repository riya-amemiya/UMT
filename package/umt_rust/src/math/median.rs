/// Calculates the median of an array of numbers.
///
/// The median is the middle value when the numbers are sorted.
/// For arrays with even length, returns the average of the two middle values.
///
/// # Arguments
///
/// * `array` - A slice of f64 numbers.
///
/// # Returns
///
/// The median value, or NaN for empty arrays.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_median;
///
/// assert_eq!(umt_median(&[1.0, 3.0, 3.0, 6.0, 7.0, 8.0, 9.0]), 6.0);
/// assert_eq!(umt_median(&[1.0, 2.0, 3.0, 4.0]), 2.5);
/// ```
pub fn umt_median(array: &[f64]) -> f64 {
    if array.is_empty() {
        return f64::NAN;
    }

    let mut sorted: Vec<f64> = array.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let mid = sorted.len() / 2;

    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_odd() {
        assert_eq!(umt_median(&[1.0, 3.0, 3.0, 6.0, 7.0, 8.0, 9.0]), 6.0);
    }

    #[test]
    fn test_median_even() {
        assert_eq!(umt_median(&[1.0, 2.0, 3.0, 4.0]), 2.5);
    }

    #[test]
    fn test_median_empty() {
        assert!(umt_median(&[]).is_nan());
    }

    #[test]
    fn test_median_unsorted() {
        assert_eq!(umt_median(&[9.0, 1.0, 5.0, 3.0, 6.0]), 5.0);
    }

    #[test]
    fn test_median_single() {
        assert_eq!(umt_median(&[5.0]), 5.0);
        assert_eq!(umt_median(&[-10.0]), -10.0);
    }

    #[test]
    fn test_median_two_elements() {
        assert_eq!(umt_median(&[1.0, 3.0]), 2.0);
        assert_eq!(umt_median(&[10.0, 20.0]), 15.0);
    }

    #[test]
    fn test_median_with_infinity() {
        assert_eq!(umt_median(&[1.0, 2.0, f64::INFINITY]), 2.0);
        assert_eq!(umt_median(&[f64::NEG_INFINITY, 0.0, 1.0]), 0.0);
    }

    #[test]
    fn test_median_same_values() {
        assert_eq!(umt_median(&[5.0, 5.0, 5.0, 5.0]), 5.0);
    }

    #[test]
    fn test_median_negative() {
        assert_eq!(umt_median(&[-5.0, -3.0, -1.0, 0.0, 2.0]), -1.0);
    }
}
