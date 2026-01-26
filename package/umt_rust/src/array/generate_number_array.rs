use crate::internal::rng;

/// Generates an array of numbers with the specified length.
///
/// # Arguments
///
/// * `length` - The length of the array
/// * `min` - The minimum value (default: 0.0)
/// * `max` - The maximum value (default: length - 1)
/// * `random` - Whether to generate random values (default: false)
///
/// # Returns
///
/// Vector of numbers
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_generate_number_array;
///
/// assert_eq!(umt_generate_number_array(5, None, None, false), vec![0.0, 1.0, 2.0, 3.0, 4.0]);
/// assert_eq!(umt_generate_number_array(5, Some(10.0), Some(14.0), false), vec![10.0, 11.0, 12.0, 13.0, 14.0]);
/// ```
pub fn umt_generate_number_array(
    length: usize,
    min: Option<f64>,
    max: Option<f64>,
    random: bool,
) -> Vec<f64> {
    if length == 0 {
        return vec![];
    }

    let actual_min = min.unwrap_or(0.0);
    let actual_max = max.unwrap_or((length - 1) as f64);

    if actual_min > actual_max {
        panic!("min should be less than or equal to max");
    }

    if length == 1 {
        return vec![actual_min];
    }

    if random {
        (0..length)
            .map(|_| {
                let value = actual_min + rng::random_f64() * (actual_max - actual_min + 1.0);
                value.floor()
            })
            .collect()
    } else {
        let step = (actual_max - actual_min) / (length - 1) as f64;
        (0..length).map(|i| actual_min + i as f64 * step).collect()
    }
}

/// Generates an array of integers with the specified length.
///
/// # Arguments
///
/// * `length` - The length of the array
/// * `min` - The minimum value (default: 0)
/// * `max` - The maximum value (default: length - 1)
/// * `random` - Whether to generate random values (default: false)
///
/// # Returns
///
/// Vector of integers
pub fn umt_generate_number_array_i32(
    length: usize,
    min: Option<i32>,
    max: Option<i32>,
    random: bool,
) -> Vec<i32> {
    if length == 0 {
        return vec![];
    }

    let actual_min = min.unwrap_or(0);
    let actual_max = max.unwrap_or((length - 1) as i32);

    if actual_min > actual_max {
        panic!("min should be less than or equal to max");
    }

    if length == 1 {
        return vec![actual_min];
    }

    if random {
        (0..length)
            .map(|_| rng::random_range_i64(actual_min as i64, actual_max as i64) as i32)
            .collect()
    } else {
        let step = (actual_max - actual_min) as f64 / (length - 1) as f64;
        (0..length)
            .map(|i| actual_min + (i as f64 * step).round() as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_basic() {
        assert_eq!(
            umt_generate_number_array(5, None, None, false),
            vec![0.0, 1.0, 2.0, 3.0, 4.0]
        );
    }

    #[test]
    fn test_generate_with_range() {
        assert_eq!(
            umt_generate_number_array(5, Some(10.0), Some(14.0), false),
            vec![10.0, 11.0, 12.0, 13.0, 14.0]
        );
    }

    #[test]
    fn test_generate_empty() {
        assert_eq!(
            umt_generate_number_array(0, None, None, false),
            Vec::<f64>::new()
        );
    }

    #[test]
    fn test_generate_single() {
        assert_eq!(
            umt_generate_number_array(1, Some(5.0), Some(10.0), false),
            vec![5.0]
        );
    }

    #[test]
    fn test_generate_random_length() {
        let result = umt_generate_number_array(10, Some(0.0), Some(100.0), true);
        assert_eq!(result.len(), 10);
        for val in &result {
            assert!(*val >= 0.0 && *val <= 100.0);
        }
    }

    #[test]
    fn test_generate_i32_basic() {
        assert_eq!(
            umt_generate_number_array_i32(5, None, None, false),
            vec![0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn test_generate_i32_with_range() {
        let result = umt_generate_number_array_i32(5, Some(10), Some(20), false);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 10);
        assert_eq!(result[4], 20);
    }

    #[test]
    fn test_generate_i32_random() {
        let result = umt_generate_number_array_i32(10, Some(0), Some(100), true);
        assert_eq!(result.len(), 10);
        for val in &result {
            assert!(*val >= 0 && *val <= 100);
        }
    }

    #[test]
    #[should_panic(expected = "min should be less than or equal to max")]
    fn test_generate_invalid_range() {
        umt_generate_number_array(5, Some(10.0), Some(5.0), false);
    }
}
