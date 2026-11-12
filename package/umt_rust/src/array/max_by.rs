/// Returns the element whose selected value is the largest.
///
/// Values are compared with `>`, so the first element wins ties and a key that
/// never compares as larger (such as NaN) never replaces the current maximum.
///
/// # Arguments
///
/// * `array` - The array to scan
/// * `selector` - Function returning the value to compare
///
/// # Returns
///
/// The element with the largest selected value, or None for an empty array
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_max_by;
///
/// assert_eq!(umt_max_by(&[(1, 'a'), (2, 'b'), (2, 'c')], |p| p.0), Some((2, 'b')));
/// assert_eq!(umt_max_by(&[] as &[i32], |n| *n), None);
/// ```
pub fn umt_max_by<T, K, F>(array: &[T], selector: F) -> Option<T>
where
    T: Clone,
    K: PartialOrd,
    F: Fn(&T) -> K,
{
    let (first, rest) = array.split_first()?;
    let mut best = first;
    let mut best_key = selector(first);
    for item in rest {
        let key = selector(item);
        if key > best_key {
            best = item;
            best_key = key;
        }
    }
    Some(best.clone())
}
