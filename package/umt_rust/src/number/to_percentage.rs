/// Calculates the percentage of a value relative to a total.
///
/// Returns 0.0 when the total is 0.0 to avoid division by zero.
///
/// # Arguments
///
/// * `value` - The partial value
/// * `total` - The total value
/// * `decimals` - The number of decimal places
///
/// # Returns
///
/// * The percentage value
pub fn umt_to_percentage(value: f64, total: f64, decimals: i32) -> f64 {
    if total == 0.0 {
        return 0.0;
    }

    let factor = 10_f64.powi(decimals);
    ((value / total) * 100.0 * factor + 0.5).floor() / factor
}

/// Calculates the percentage of a value relative to a total with 2 decimal places by default.
///
/// Returns 0.0 when the total is 0.0 to avoid division by zero.
///
/// # Arguments
///
/// * `value` - The partial value
/// * `total` - The total value
///
/// # Returns
///
/// * The percentage value
pub fn umt_to_percentage_default(value: f64, total: f64) -> f64 {
    umt_to_percentage(value, total, 2)
}
