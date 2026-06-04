use rand::RngExt;

/// Returns a random integer in the closed interval `[ceil(min), floor(max)]`.
///
/// The lower bound is ceiled and the upper bound is floored before sampling,
/// matching the TypeScript `randomInt` behavior. The result is returned as an
/// integer-valued `f64` to mirror the JavaScript `number` semantics.
///
/// # Arguments
///
/// * `min` - Inclusive lower bound (ceiled).
/// * `max` - Inclusive upper bound (floored).
///
/// # Returns
///
/// Random integer in `[ceil(min), floor(max)]`.
///
/// # Examples
///
/// ```
/// use umt_rust::random::umt_random_int;
///
/// let value = umt_random_int(1.0, 6.0);
/// assert!(value >= 1.0 && value <= 6.0);
///
/// assert_eq!(umt_random_int(3.0, 3.0), 3.0);
///
/// let value = umt_random_int(0.4, 5.7);
/// assert!(value >= 1.0 && value <= 5.0);
/// ```
pub fn umt_random_int(min: f64, max: f64) -> f64 {
    let lo = min.ceil();
    let hi = max.floor();
    let mut rng = rand::rng();
    let value: f64 = rng.random();
    (value * (hi - lo + 1.0)).floor() + lo
}
