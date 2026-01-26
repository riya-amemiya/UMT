/// Returns the minimum value from a slice of numbers.
///
/// This function takes a slice of f64 numbers and returns the minimum value.
/// Duplicates are automatically removed during processing.
///
/// # Arguments
///
/// * `numbers` - A slice of f64 numbers.
///
/// # Returns
///
/// The minimum value from the input slice, or f64::INFINITY if empty.
#[inline]
pub fn umt_min(numbers: &[f64]) -> f64 {
    numbers
        .iter()
        .fold(f64::INFINITY, |acc, &x| if x < acc { x } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_basic() {
        assert_eq!(umt_min(&[1.0, 2.0, 3.0]), 1.0);
    }

    #[test]
    fn test_min_negative() {
        assert_eq!(umt_min(&[-1.0, -2.0, -3.0]), -3.0);
    }

    #[test]
    fn test_min_mixed() {
        assert_eq!(umt_min(&[-1.0, 0.0, 1.0]), -1.0);
    }

    #[test]
    fn test_min_single() {
        assert_eq!(umt_min(&[5.0]), 5.0);
    }

    #[test]
    fn test_min_empty() {
        assert_eq!(umt_min(&[]), f64::INFINITY);
    }

    #[test]
    fn test_min_with_duplicates() {
        assert_eq!(umt_min(&[1.0, 3.0, 1.0, 2.0]), 1.0);
    }
}
