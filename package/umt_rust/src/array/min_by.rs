/// Returns the element whose selected value is the smallest.
///
/// Values are compared with `<`, so the first element wins ties and a key that
/// never compares as smaller (such as NaN) never replaces the current minimum.
///
/// # Arguments
///
/// * `array` - The array to scan
/// * `selector` - Function returning the value to compare
///
/// # Returns
///
/// The element with the smallest selected value, or None for an empty array
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_min_by;
///
/// assert_eq!(umt_min_by(&[(2, 'a'), (1, 'b'), (1, 'c')], |p| p.0), Some((1, 'b')));
/// assert_eq!(umt_min_by(&[] as &[i32], |n| *n), None);
/// ```
pub fn umt_min_by<T, K, F>(array: &[T], selector: F) -> Option<T>
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
        if key < best_key {
            best = item;
            best_key = key;
        }
    }
    Some(best.clone())
}
