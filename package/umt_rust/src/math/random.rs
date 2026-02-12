use rand::RngExt;

/// Generates a random integer between min and max (inclusive).
///
/// # Arguments
///
/// * `max` - Maximum value (inclusive).
/// * `min` - Minimum value (inclusive, default: 0).
///
/// # Returns
///
/// Random integer between min and max.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_random;
///
/// let r = umt_random(10, 0);
/// assert!(r >= 0 && r <= 10);
///
/// let r = umt_random(10, 5);
/// assert!(r >= 5 && r <= 10);
/// ```
pub fn umt_random(max: i64, min: i64) -> i64 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

/// Generates a random integer between 0 and max (inclusive).
///
/// # Arguments
///
/// * `max` - Maximum value (inclusive).
///
/// # Returns
///
/// Random integer between 0 and max.
#[inline]
pub fn umt_random_max(max: i64) -> i64 {
    umt_random(max, 0)
}
