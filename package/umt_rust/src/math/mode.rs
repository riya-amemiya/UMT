use std::collections::HashMap;

/// Finds the most frequently occurring value(s) in an array.
///
/// If there are ties, returns all values with the maximum frequency.
///
/// # Arguments
///
/// * `array` - A slice of f64 numbers.
///
/// # Returns
///
/// A sorted vector of mode values (can be multiple values if there are ties).
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_mode;
///
/// assert_eq!(umt_mode(&[1.0, 2.0, 2.0, 3.0, 3.0, 3.0]), vec![3.0]);
/// assert_eq!(umt_mode(&[1.0, 2.0, 2.0, 3.0, 3.0]), vec![2.0, 3.0]);
/// assert_eq!(umt_mode(&[1.0, 2.0, 3.0]), vec![1.0, 2.0, 3.0]);
/// ```
pub fn umt_mode(array: &[f64]) -> Vec<f64> {
    if array.is_empty() {
        return vec![];
    }

    // Use ordered bits as key for HashMap to handle f64
    let mut frequency: HashMap<u64, (f64, usize)> = HashMap::new();
    let mut max_frequency = 0;

    // Count frequencies
    for &value in array {
        let key = value.to_bits();
        let entry = frequency.entry(key).or_insert((value, 0));
        entry.1 += 1;
        if entry.1 > max_frequency {
            max_frequency = entry.1;
        }
    }

    // Find all values with maximum frequency
    let mut modes: Vec<f64> = frequency
        .values()
        .filter(|(_, count)| *count == max_frequency)
        .map(|(value, _)| *value)
        .collect();

    // Sort the result
    modes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    modes
}
