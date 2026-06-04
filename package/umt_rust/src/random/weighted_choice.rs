use rand::RngExt;

/// An item paired with a weight for [`umt_weighted_choice`].
#[derive(Debug, Clone, PartialEq)]
pub struct WeightedItem<T> {
    pub value: T,
    pub weight: f64,
}

/// Returns a random element using cumulative weights and binary search.
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

    let mut cumulative: Vec<f64> = Vec::with_capacity(items.len());
    let mut total = 0.0;
    for item in items {
        if item.weight > 0.0 {
            total += item.weight;
        }
        cumulative.push(total);
    }

    let mut rng = rand::rng();
    let value: f64 = rng.random();
    let target = value * total;

    let mut lo = 0usize;
    let mut hi = cumulative.len() - 1;
    while lo < hi {
        let mid = (lo + hi) >> 1;
        if cumulative[mid] <= target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    Some(items[lo].value.clone())
}
