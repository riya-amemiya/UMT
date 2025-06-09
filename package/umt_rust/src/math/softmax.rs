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
    let mut max = f64::NEG_INFINITY;
    for &i in &x {
        if i > max {
            max = i;
        }
    }

    let mut exp: Vec<f64> = Vec::with_capacity(x.len());
    for &i in &x {
        exp.push((i - max).exp());
    }

    let mut sum = 0.0;
    for &i in &exp {
        sum += i;
    }

    let mut result: Vec<f64> = Vec::with_capacity(x.len());
    for &i in &exp {
        result.push(umt_round_of(i / sum, decimal_place));
    }
    result
}
