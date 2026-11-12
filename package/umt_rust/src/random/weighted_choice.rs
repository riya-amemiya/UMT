use rand::RngExt;

/// An item paired with a weight for [`umt_weighted_choice`].
#[derive(Debug, Clone, PartialEq)]
pub struct WeightedItem<T> {
    pub value: T,
    pub weight: f64,
}

/// Returns the value whose cumulative weight range contains `target`.
///
/// Walks the running total of the positive weights and returns the first value
/// whose running total exceeds `target`, falling back to the last value when
/// none does. A NaN `target` selects the first value. This is the selection
/// step of [`umt_weighted_choice`] with the sampled point supplied by the
/// caller.
///
/// # Arguments
///
/// * `items` - Items with weights.
/// * `target` - A point in `[0, total weight)`.
///
/// # Returns
///
/// A reference to the selected value, or `None` if the slice is empty.
///
/// # Examples
///
/// ```
/// use umt_rust::random::{umt_weighted_choice_at, WeightedItem};
///
/// let items = vec![
///     WeightedItem { value: "a", weight: 1.0 },
///     WeightedItem { value: "b", weight: 4.0 },
/// ];
/// assert_eq!(umt_weighted_choice_at(&items, 0.5), Some(&"a"));
/// assert_eq!(umt_weighted_choice_at(&items, 1.0), Some(&"b"));
/// ```
pub fn umt_weighted_choice_at<T>(items: &[WeightedItem<T>], target: f64) -> Option<&T> {
    let (last, rest) = items.split_last()?;
    let mut cumulative = 0.0;
    for item in rest {
        if item.weight > 0.0 {
            cumulative += item.weight;
        }
        if cumulative > target || target.is_nan() {
            return Some(&item.value);
        }
    }
    Some(&last.value)
}

/// Returns a random element by sampling a point on the cumulative weights.
///
/// Items with non-positive weights are skipped from the pool (their weight does
/// not contribute to the total). Mirrors the TypeScript `weightedChoice`, which
/// assumes a non-empty input; returns `None` only when the slice is empty.
///
/// # Arguments
///
/// * `items` - Items with weights.
///
/// # Returns
///
/// A randomly selected value, or `None` if the slice is empty.
///
/// # Examples
///
/// ```
/// use umt_rust::random::{umt_weighted_choice, WeightedItem};
///
/// let items = vec![
///     WeightedItem { value: "skip", weight: 0.0 },
///     WeightedItem { value: "winner", weight: 5.0 },
/// ];
/// assert_eq!(umt_weighted_choice(&items), Some("winner"));
/// ```
pub fn umt_weighted_choice<T: Clone>(items: &[WeightedItem<T>]) -> Option<T> {
    if items.is_empty() {
        return None;
    }

    let mut total = 0.0;
    for item in items {
        if item.weight > 0.0 {
            total += item.weight;
        }
    }

    let mut rng = rand::rng();
    let value: f64 = rng.random();
    Some(umt_weighted_choice_at(items, value * total)?.clone())
}
