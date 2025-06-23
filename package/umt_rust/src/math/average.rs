/// Calculates the average of a vector of numbers.
///
/// This function takes a vector of f64 numbers as input and returns the average of those numbers.
///
/// # Arguments
///
/// * `numbers` - A vector of f64 numbers.
///
/// # Returns
///
/// The average of the numbers in the input vector.
pub fn umt_average(numbers: Vec<f64>) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}
