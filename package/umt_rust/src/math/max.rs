/// Returns the maximum value from a slice of numbers.
///
/// This function takes a slice of f64 numbers and returns the maximum value.
/// Duplicates are automatically removed during processing.
///
/// # Arguments
///
/// * `numbers` - A slice of f64 numbers.
///
/// # Returns
///
/// The maximum value from the input slice, or f64::NEG_INFINITY if empty.
#[inline]
pub fn umt_max(numbers: &[f64]) -> f64 {
    numbers
        .iter()
        .fold(f64::NEG_INFINITY, |acc, &x| if x > acc { x } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_basic() {
        assert_eq!(umt_max(&[1.0, 2.0, 3.0]), 3.0);
    }

    #[test]
    fn test_max_negative() {
        assert_eq!(umt_max(&[-1.0, -2.0, -3.0]), -1.0);
    }

    #[test]
    fn test_max_mixed() {
        assert_eq!(umt_max(&[-1.0, 0.0, 1.0]), 1.0);
    }

    #[test]
    fn test_max_single() {
        assert_eq!(umt_max(&[5.0]), 5.0);
    }

    #[test]
    fn test_max_empty() {
        assert_eq!(umt_max(&[]), f64::NEG_INFINITY);
    }

    #[test]
    fn test_max_with_duplicates() {
        assert_eq!(umt_max(&[1.0, 3.0, 3.0, 2.0]), 3.0);
    }
}
