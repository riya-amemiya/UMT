use rand::RngExt;

/// Returns a random float in the half-open interval `[min, max)`.
///
/// When `min` equals `max`, the result is exactly `min`.
///
/// # Arguments
///
/// * `min` - Inclusive lower bound.
/// * `max` - Exclusive upper bound.
///
/// # Returns
///
/// Random float.
///
/// # Examples
///
/// ```
/// use umt_rust::random::umt_random_float;
///
/// let value = umt_random_float(0.0, 1.0);
/// assert!(value >= 0.0 && value < 1.0);
///
/// assert_eq!(umt_random_float(7.0, 7.0), 7.0);
/// ```
pub fn umt_random_float(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    let value: f64 = rng.random();
    value * (max - min) + min
}
