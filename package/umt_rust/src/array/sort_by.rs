use super::compare_function_default::umt_compare_function_default;
use super::tim_sort::tim_sort_range;

/// Sorts an array by one or more key selectors with a stable TimSort.
///
/// Each selector is called once per element. Later selectors break ties left by
/// earlier ones, and elements whose keys are all equal keep their original order.
/// Keys are compared with `umt_compare_function_default`, so incomparable keys
/// such as NaN compare as equal. All selectors return the same key type `K`.
///
/// # Arguments
///
/// * `array` - The array to sort (not mutated)
/// * `selectors` - Functions returning the sort keys, in priority order
///
/// # Returns
///
/// A new sorted vector
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_sort_by;
///
/// let words = vec!["bb", "a", "ab"];
/// assert_eq!(umt_sort_by(&words, &[&|s: &&str| s.len()]), vec!["a", "bb", "ab"]);
///
/// let pairs = vec![(1, 2), (0, 9), (1, 1)];
/// let sorted = umt_sort_by(&pairs, &[&|p: &(i32, i32)| p.0, &|p: &(i32, i32)| p.1]);
/// assert_eq!(sorted, vec![(0, 9), (1, 1), (1, 2)]);
/// ```
pub fn umt_sort_by<T, K>(array: &[T], selectors: &[&dyn Fn(&T) -> K]) -> Vec<T>
where
    T: Clone,
    K: PartialOrd,
{
    if array.is_empty() {
        return vec![];
    }

    let keys: Vec<Vec<K>> = array
        .iter()
        .map(|item| selectors.iter().map(|selector| selector(item)).collect())
        .collect();

    let mut order: Vec<usize> = (0..array.len()).collect();
    let compare = |a: &usize, b: &usize| -> i32 {
        for (key_a, key_b) in keys[*a].iter().zip(keys[*b].iter()) {
            let result = umt_compare_function_default(key_a, key_b);
            if result != 0 {
                return result;
            }
        }
        0
    };
    let end = order.len() - 1;
    tim_sort_range(&mut order, &compare, 0, end);

    order
        .into_iter()
        .map(|index| array[index].clone())
        .collect()
}
