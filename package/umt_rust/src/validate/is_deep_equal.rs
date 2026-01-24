//! Deep equality comparison utility

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Options for deep equality comparison
#[derive(Debug, Clone)]
pub struct IsDeepEqualOptions {
    /// Whether to require strict array order when comparing arrays
    /// Default: true
    pub strict_order: bool,
}

impl Default for IsDeepEqualOptions {
    fn default() -> Self {
        Self { strict_order: true }
    }
}

/// Performs a deep equality comparison between two values
///
/// # Arguments
/// * `a` - First value to compare
/// * `b` - Second value to compare
/// * `options` - Comparison options
///
/// # Returns
/// true if values are deeply equal, false otherwise
///
/// # Examples
/// ```
/// use umt_rust::validate::{umt_is_deep_equal, IsDeepEqualOptions};
///
/// assert!(umt_is_deep_equal(&vec![1, 2, 3], &vec![1, 2, 3], None));
/// assert!(!umt_is_deep_equal(&vec![1, 2, 3], &vec![3, 2, 1], None));
///
/// let opts = IsDeepEqualOptions { strict_order: false };
/// assert!(umt_is_deep_equal(&vec![1, 2, 3], &vec![3, 2, 1], Some(opts)));
/// ```
#[inline]
pub fn umt_is_deep_equal<T: PartialEq + Clone>(
    a: &Vec<T>,
    b: &Vec<T>,
    options: Option<IsDeepEqualOptions>,
) -> bool {
    let opts = options.unwrap_or_default();

    if a.len() != b.len() {
        return false;
    }

    if opts.strict_order {
        a == b
    } else {
        let mut b_copy = b.clone();
        for item in a {
            if let Some(pos) = b_copy.iter().position(|x| x == item) {
                b_copy.remove(pos);
            } else {
                return false;
            }
        }
        b_copy.is_empty()
    }
}

/// Deep equality for HashMaps
#[inline]
pub fn umt_is_deep_equal_map<K, V>(a: &HashMap<K, V>, b: &HashMap<K, V>) -> bool
where
    K: Eq + Hash,
    V: PartialEq,
{
    if a.len() != b.len() {
        return false;
    }

    for (key, value) in a {
        match b.get(key) {
            Some(b_value) if value == b_value => continue,
            _ => return false,
        }
    }
    true
}

/// Deep equality for HashSets
#[inline]
pub fn umt_is_deep_equal_set<T>(a: &HashSet<T>, b: &HashSet<T>) -> bool
where
    T: Eq + Hash,
{
    a == b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_deep_equal_strict_order() {
        assert!(umt_is_deep_equal(&vec![1, 2, 3], &vec![1, 2, 3], None));
        assert!(!umt_is_deep_equal(&vec![1, 2, 3], &vec![3, 2, 1], None));
    }

    #[test]
    fn test_is_deep_equal_unordered() {
        let opts = IsDeepEqualOptions { strict_order: false };
        assert!(umt_is_deep_equal(&vec![1, 2, 3], &vec![3, 2, 1], Some(opts)));
    }

    #[test]
    fn test_is_deep_equal_map() {
        let mut a = HashMap::new();
        a.insert("key", 1);
        let mut b = HashMap::new();
        b.insert("key", 1);
        assert!(umt_is_deep_equal_map(&a, &b));
    }

    #[test]
    fn test_is_deep_equal_set() {
        let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let b: HashSet<i32> = [3, 2, 1].into_iter().collect();
        assert!(umt_is_deep_equal_set(&a, &b));
    }
}
