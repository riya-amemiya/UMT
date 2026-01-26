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
