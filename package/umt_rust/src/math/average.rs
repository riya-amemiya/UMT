use super::{umt_addition, umt_division};

/// Calculates the average of a vector of numbers.
///
/// Uses precision-corrected addition and division to avoid floating point errors.
/// Returns 0.0 for empty vectors.
///
/// # Arguments
///
/// * `numbers` - A vector of f64 numbers.
///
/// # Returns
///
/// The average of the numbers in the input vector.
pub fn umt_average(numbers: Vec<f64>) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }

    let sum = umt_addition(&numbers);
    umt_division(sum, numbers.len() as f64)
}
