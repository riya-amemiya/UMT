use rand::RngExt;

/// Returns a random boolean. The probability of `true` defaults to 0.5.
///
/// # Arguments
///
/// * `probability` - Probability of returning true (0..1).
///
/// # Returns
///
/// Random boolean.
///
/// # Examples
///
/// ```
/// use umt_rust::random::umt_random_boolean;
///
/// let _ = umt_random_boolean(0.5); // true or false equally
/// assert_eq!(umt_random_boolean(1.0), true); // always true
/// assert_eq!(umt_random_boolean(0.0), false); // always false
/// ```
pub fn umt_random_boolean(probability: f64) -> bool {
    let mut rng = rand::rng();
    let value: f64 = rng.random();
    value < probability
}

/// Returns a random boolean with the default probability of 0.5.
///
/// # Returns
///
/// Random boolean.
#[inline]
pub fn umt_random_boolean_default() -> bool {
    umt_random_boolean(0.5)
}
