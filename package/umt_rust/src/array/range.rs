/// Generates an array of sequential numbers.
///
/// # Arguments
///
/// * `start` - Starting number (or end if `end` is None)
/// * `end` - Ending number (exclusive). If None, generates array from 0 to `start`
/// * `step` - Step value (defaults to 1)
///
/// # Returns
///
/// Array of sequential numbers
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_range;
///
/// assert_eq!(umt_range(5, None, None), vec![0, 1, 2, 3, 4]);
/// assert_eq!(umt_range(2, Some(10), Some(2)), vec![2, 4, 6, 8]);
/// assert_eq!(umt_range(5, Some(1), Some(-1)), vec![5, 4, 3, 2]);
/// ```
pub fn umt_range(start: i32, end: Option<i32>, step: Option<i32>) -> Vec<i32> {
    let step_val = step.unwrap_or(1);

    // Handle invalid step
    if step_val == 0 {
        return vec![];
    }

    let (actual_start, actual_end) = match end {
        Some(e) => (start, e),
        None => (0, start),
    };

    // Return empty array if invalid range
    if (step_val > 0 && actual_start >= actual_end) || (step_val < 0 && actual_start <= actual_end)
    {
        return vec![];
    }

    let mut result = Vec::new();

    if step_val > 0 {
        let mut index = actual_start;
        while index < actual_end {
            result.push(index);
            index += step_val;
        }
    } else {
        let mut index = actual_start;
        while index > actual_end {
            result.push(index);
            index += step_val;
        }
    }

    result
}

/// Generates an array of sequential floating-point numbers with floating point precision handling.
///
/// # Arguments
///
/// * `start` - Starting number (or end if `end` is None)
/// * `end` - Ending number (exclusive). If None, generates array from 0 to `start`
/// * `step` - Step value (defaults to 1.0)
///
/// # Returns
///
/// Array of sequential numbers
pub fn umt_range_f64(start: f64, end: Option<f64>, step: Option<f64>) -> Vec<f64> {
    let step_val = step.unwrap_or(1.0);

    // Handle invalid step
    if step_val == 0.0 {
        return vec![];
    }

    let (actual_start, actual_end) = match end {
        Some(e) => (start, e),
        None => (0.0, start),
    };

    // Return empty array if invalid range
    if (step_val > 0.0 && actual_start >= actual_end)
        || (step_val < 0.0 && actual_start <= actual_end)
    {
        return vec![];
    }

    let mut result = Vec::new();

    if step_val > 0.0 {
        let mut index = actual_start;
        while index < actual_end {
            // Handle floating point precision
            let rounded = (index * 1e10).round() / 1e10;
            result.push(rounded);
            index += step_val;
        }
    } else {
        let mut index = actual_start;
        while index > actual_end {
            // Handle floating point precision
            let rounded = (index * 1e10).round() / 1e10;
            result.push(rounded);
            index += step_val;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_single_arg() {
        assert_eq!(umt_range(5, None, None), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_range_start_end() {
        assert_eq!(umt_range(2, Some(6), None), vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_range_with_step() {
        assert_eq!(umt_range(2, Some(10), Some(2)), vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_range_negative_step() {
        assert_eq!(umt_range(5, Some(1), Some(-1)), vec![5, 4, 3, 2]);
    }

    #[test]
    fn test_range_zero_step() {
        assert_eq!(umt_range(5, Some(10), Some(0)), Vec::<i32>::new());
    }

    #[test]
    fn test_range_invalid_range_positive_step() {
        assert_eq!(umt_range(10, Some(5), Some(1)), Vec::<i32>::new());
    }

    #[test]
    fn test_range_invalid_range_negative_step() {
        assert_eq!(umt_range(5, Some(10), Some(-1)), Vec::<i32>::new());
    }

    #[test]
    fn test_range_empty_when_equal() {
        assert_eq!(umt_range(5, Some(5), None), Vec::<i32>::new());
    }

    #[test]
    fn test_range_negative_numbers() {
        assert_eq!(umt_range(-3, Some(3), None), vec![-3, -2, -1, 0, 1, 2]);
    }

    #[test]
    fn test_range_f64_basic() {
        let result = umt_range_f64(5.0, None, None);
        assert_eq!(result, vec![0.0, 1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_range_f64_with_step() {
        let result = umt_range_f64(0.0, Some(1.0), Some(0.2));
        assert_eq!(result.len(), 5);
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - 0.2).abs() < 1e-10);
        assert!((result[2] - 0.4).abs() < 1e-10);
        assert!((result[3] - 0.6).abs() < 1e-10);
        assert!((result[4] - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_range_f64_negative_step() {
        let result = umt_range_f64(1.0, Some(0.0), Some(-0.2));
        assert_eq!(result.len(), 5);
        assert!((result[0] - 1.0).abs() < 1e-10);
        assert!((result[4] - 0.2).abs() < 1e-10);
    }
}
