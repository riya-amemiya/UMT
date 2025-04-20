use super::umt_round_of;

/// Calculates the softmax of a vector of numbers.
///
/// This function takes a vector of numbers and a decimal place as input and returns the softmax of the numbers, rounded to the specified decimal place.
///
/// # Arguments
///
/// * `x` - The vector of numbers.
/// * `decimal_place` - The decimal place to round to.
///
/// # Returns
///
/// The softmax of the numbers, rounded to the specified decimal place.
pub fn umt_softmax(x: Vec<f64>, decimal_place: i32) -> Vec<f64> {
    let max = x.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let exp: Vec<f64> = x.iter().map(|i| (i - max).exp()).collect();
    let sum = exp.iter().fold(0.0, |a, &b| a + b);
    exp.iter()
        .map(|i| umt_round_of(i / sum, decimal_place))
        .collect()
}
