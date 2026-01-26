/// Generates an array of multiples of a number.
///
/// # Arguments
///
/// * `x` - Base number.
/// * `n` - Number of multiples to generate.
///
/// # Returns
///
/// A vector of n multiples of x.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_multiples;
///
/// assert_eq!(umt_multiples(2.0, 5), vec![2.0, 4.0, 6.0, 8.0, 10.0]);
/// ```
pub fn umt_multiples(x: f64, n: usize) -> Vec<f64> {
    (1..=n).map(|i| x * i as f64).collect()
}
