use crate::math::{umt_addition, umt_division, umt_multiplication, umt_subtract};
use rand::RngExt;

const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

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

    if length == 1 {
        return vec![actual_min];
    }

    let is_integer_inputs = is_safe_integer(actual_min) && is_safe_integer(actual_max);

    if random {
        let mut rng = rand::rng();
        if is_integer_inputs {
            let range = actual_max - actual_min + 1.0;
            return (0..length)
                .map(|_| (rng.random::<f64>() * range).floor() + actual_min)
                .collect();
        }
        let range = umt_addition(&[umt_subtract(&[actual_max, actual_min]), 1.0]);
        return (0..length)
            .map(|_| {
                umt_addition(&[
                    umt_multiplication(&[rng.random::<f64>(), range]).floor(),
                    actual_min,
                ])
            })
            .collect();
    }

    let steps = (length - 1) as f64;

    if is_integer_inputs {
        let range = actual_max - actual_min;
        if range % steps == 0.0 {
            let step = range / steps;
            return (0..length).map(|i| actual_min + i as f64 * step).collect();
        }
    }

    let step = umt_division(umt_subtract(&[actual_max, actual_min]), steps);
    (0..length)
        .map(|i| umt_addition(&[actual_min, umt_multiplication(&[i as f64, step])]))
        .collect()
}

fn is_safe_integer(value: f64) -> bool {
    value.fract() == 0.0 && value.abs() <= MAX_SAFE_INTEGER
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

    if length == 1 {
        return vec![actual_min];
    }

    if random {
        let mut rng = rand::rng();
        let (low, high) = if actual_min <= actual_max {
            (actual_min, actual_max)
        } else if actual_min - actual_max == 1 {
            (actual_min, actual_min)
        } else {
            (actual_max + 1, actual_min - 1)
        };
        (0..length).map(|_| rng.random_range(low..=high)).collect()
    } else {
        let step = (actual_max - actual_min) as f64 / (length - 1) as f64;
        (0..length)
            .map(|i| actual_min + (i as f64 * step).round() as i32)
            .collect()
    }
}
