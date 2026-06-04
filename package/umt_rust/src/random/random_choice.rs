use rand::RngExt;

/// Returns a uniformly random element from the input slice.
///
/// Mirrors the TypeScript `randomChoice`, which assumes a non-empty array.
/// Returns `None` only when the slice is empty.
///
/// # Arguments
///
/// * `items` - Source slice (expected to be non-empty).
///
/// # Returns
///
/// A randomly selected element, or `None` if the slice is empty.
///
/// # Examples
///
/// ```
/// use umt_rust::random::umt_random_choice;
///
/// let picked = umt_random_choice(&["a", "b", "c"]);
/// assert!(picked.is_some());
///
/// let only = umt_random_choice(&[42]);
/// assert_eq!(only, Some(42));
/// ```
pub fn umt_random_choice<T: Clone>(items: &[T]) -> Option<T> {
    if items.is_empty() {
        return None;
    }
    let mut rng = rand::rng();
    let value: f64 = rng.random();
    let index = (value * items.len() as f64).floor() as usize;
    Some(items[index].clone())
}
