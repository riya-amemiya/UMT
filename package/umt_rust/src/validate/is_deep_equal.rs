//! Deep equality comparison functionality
//!
//! This module provides a function to perform deep equality comparison between two values.

use super::types::DynamicValue;
use std::collections::{HashMap, HashSet};

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
///
/// * `a` - First value to compare
/// * `b` - Second value to compare
/// * `options` - Optional comparison options
///
/// # Returns
///
/// `true` if values are deeply equal, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::{umt_is_deep_equal, DynamicValue, IsDeepEqualOptions};
/// use std::collections::HashMap;
///
/// // Primitive comparison
/// assert!(umt_is_deep_equal(
///     &DynamicValue::Integer(1),
///     &DynamicValue::Integer(1),
///     None
/// ));
///
/// // Array comparison
/// let arr1 = DynamicValue::Array(vec![DynamicValue::Integer(1), DynamicValue::Integer(2)]);
/// let arr2 = DynamicValue::Array(vec![DynamicValue::Integer(1), DynamicValue::Integer(2)]);
/// assert!(umt_is_deep_equal(&arr1, &arr2, None));
///
/// // Array comparison with strict_order = false
/// let arr3 = DynamicValue::Array(vec![DynamicValue::Integer(2), DynamicValue::Integer(1)]);
/// let options = IsDeepEqualOptions { strict_order: false };
/// assert!(umt_is_deep_equal(&arr1, &arr3, Some(options)));
/// ```
pub fn umt_is_deep_equal(
    a: &DynamicValue,
    b: &DynamicValue,
    options: Option<IsDeepEqualOptions>,
) -> bool {
    let options = options.unwrap_or_default();
    let mut visited: HashSet<usize> = HashSet::new();
    compare(a, b, &options, &mut visited)
}

fn compare(
    x: &DynamicValue,
    y: &DynamicValue,
    options: &IsDeepEqualOptions,
    visited: &mut HashSet<usize>,
) -> bool {
    // Check for same reference (using pointer comparison for objects)
    if std::ptr::eq(x, y) {
        return true;
    }

    // Check null values
    if x.is_null() && y.is_null() {
        return true;
    }

    if x.is_null() || y.is_null() {
        return false;
    }

    // Check if types are the same
    if std::mem::discriminant(x) != std::mem::discriminant(y) {
        return false;
    }

    match (x, y) {
        (DynamicValue::Bool(a), DynamicValue::Bool(b)) => a == b,
        (DynamicValue::Integer(a), DynamicValue::Integer(b)) => a == b,
        (DynamicValue::Float(a), DynamicValue::Float(b)) => {
            // Handle NaN comparison (NaN == NaN should be true for deep equality)
            if a.is_nan() && b.is_nan() {
                return true;
            }
            // Handle -0 vs +0 (should be false)
            if *a == 0.0 && *b == 0.0 {
                return a.is_sign_positive() == b.is_sign_positive();
            }
            a == b
        }
        (DynamicValue::Integer(a), DynamicValue::Float(b))
        | (DynamicValue::Float(b), DynamicValue::Integer(a)) => {
            let a_float = *a as f64;
            if b.is_nan() {
                return false;
            }
            a_float == *b
        }
        (DynamicValue::String(a), DynamicValue::String(b)) => a == b,
        (DynamicValue::Array(a), DynamicValue::Array(b)) => {
            if a.len() != b.len() {
                return false;
            }

            // Check for circular reference (using pointer address)
            let ptr = a.as_ptr() as usize;
            if visited.contains(&ptr) {
                return true;
            }
            visited.insert(ptr);

            if options.strict_order {
                for (i, item_a) in a.iter().enumerate() {
                    if !compare(item_a, &b[i], options, visited) {
                        return false;
                    }
                }
            } else {
                // Non-strict order comparison
                let mut b_copy: Vec<&DynamicValue> = b.iter().collect();
                for item_a in a {
                    let mut found = false;
                    for i in 0..b_copy.len() {
                        if compare(item_a, b_copy[i], options, visited) {
                            b_copy.remove(i);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return false;
                    }
                }
                if !b_copy.is_empty() {
                    return false;
                }
            }
            true
        }
        (DynamicValue::Object(a), DynamicValue::Object(b)) => {
            if a.len() != b.len() {
                return false;
            }

            // Check for circular reference (using pointer address)
            let ptr = a as *const HashMap<String, DynamicValue> as usize;
            if visited.contains(&ptr) {
                return true;
            }
            visited.insert(ptr);

            // Check all keys exist
            for key in a.keys() {
                if !b.contains_key(key) {
                    return false;
                }
            }

            // Compare values
            for (key, value_a) in a {
                if let Some(value_b) = b.get(key) {
                    if !compare(value_a, value_b, options, visited) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

/// Deep equality comparison for generic types that implement PartialEq
///
/// # Arguments
///
/// * `a` - First value to compare
/// * `b` - Second value to compare
///
/// # Returns
///
/// `true` if values are equal, `false` otherwise
#[inline]
pub fn umt_is_deep_equal_generic<T: PartialEq>(a: &T, b: &T) -> bool {
    a == b
}

/// Deep equality comparison for f64 values using Object.is semantics
///
/// # Arguments
///
/// * `a` - First value to compare
/// * `b` - Second value to compare
///
/// # Returns
///
/// `true` if values are equal (NaN == NaN is true, -0 != +0)
#[inline]
pub fn umt_is_deep_equal_f64(a: f64, b: f64) -> bool {
    // Handle NaN comparison
    if a.is_nan() && b.is_nan() {
        return true;
    }
    // Handle -0 vs +0
    if a == 0.0 && b == 0.0 {
        return a.is_sign_positive() == b.is_sign_positive();
    }
    a == b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_comparison() {
        assert!(umt_is_deep_equal(
            &DynamicValue::Integer(1),
            &DynamicValue::Integer(1),
            None
        ));
        assert!(umt_is_deep_equal(
            &DynamicValue::String("test".to_string()),
            &DynamicValue::String("test".to_string()),
            None
        ));
        assert!(umt_is_deep_equal(
            &DynamicValue::Bool(true),
            &DynamicValue::Bool(true),
            None
        ));
        assert!(umt_is_deep_equal(
            &DynamicValue::Null,
            &DynamicValue::Null,
            None
        ));
    }

    #[test]
    fn test_different_primitive_values() {
        assert!(!umt_is_deep_equal(
            &DynamicValue::Integer(1),
            &DynamicValue::Integer(2),
            None
        ));
        assert!(!umt_is_deep_equal(
            &DynamicValue::String("test".to_string()),
            &DynamicValue::String("other".to_string()),
            None
        ));
        assert!(!umt_is_deep_equal(
            &DynamicValue::Bool(true),
            &DynamicValue::Bool(false),
            None
        ));
    }

    #[test]
    fn test_nan_comparison() {
        assert!(umt_is_deep_equal(
            &DynamicValue::Float(f64::NAN),
            &DynamicValue::Float(f64::NAN),
            None
        ));
    }

    #[test]
    fn test_negative_zero_vs_positive_zero() {
        assert!(!umt_is_deep_equal(
            &DynamicValue::Float(-0.0),
            &DynamicValue::Float(0.0),
            None
        ));
    }

    #[test]
    fn test_different_types() {
        assert!(!umt_is_deep_equal(
            &DynamicValue::Integer(1),
            &DynamicValue::String("1".to_string()),
            None
        ));
        assert!(!umt_is_deep_equal(
            &DynamicValue::Integer(0),
            &DynamicValue::Bool(false),
            None
        ));
    }

    #[test]
    fn test_array_comparison() {
        let arr1 = DynamicValue::Array(vec![
            DynamicValue::Integer(1),
            DynamicValue::Integer(2),
            DynamicValue::Integer(3),
        ]);
        let arr2 = DynamicValue::Array(vec![
            DynamicValue::Integer(1),
            DynamicValue::Integer(2),
            DynamicValue::Integer(3),
        ]);
        assert!(umt_is_deep_equal(&arr1, &arr2, None));

        let empty1 = DynamicValue::Array(vec![]);
        let empty2 = DynamicValue::Array(vec![]);
        assert!(umt_is_deep_equal(&empty1, &empty2, None));
    }

    #[test]
    fn test_array_different_elements() {
        let arr1 = DynamicValue::Array(vec![
            DynamicValue::Integer(1),
            DynamicValue::Integer(2),
            DynamicValue::Integer(3),
        ]);
        let arr2 = DynamicValue::Array(vec![
            DynamicValue::Integer(1),
            DynamicValue::Integer(2),
            DynamicValue::Integer(4),
        ]);
        assert!(!umt_is_deep_equal(&arr1, &arr2, None));
    }

    #[test]
    fn test_array_different_order_strict() {
        let arr1 = DynamicValue::Array(vec![
            DynamicValue::Integer(1),
            DynamicValue::Integer(2),
            DynamicValue::Integer(3),
        ]);
        let arr2 = DynamicValue::Array(vec![
            DynamicValue::Integer(3),
            DynamicValue::Integer(2),
            DynamicValue::Integer(1),
        ]);
        assert!(!umt_is_deep_equal(&arr1, &arr2, None));
    }

    #[test]
    fn test_array_different_order_non_strict() {
        let arr1 = DynamicValue::Array(vec![
            DynamicValue::Integer(1),
            DynamicValue::Integer(2),
            DynamicValue::Integer(3),
        ]);
        let arr2 = DynamicValue::Array(vec![
            DynamicValue::Integer(3),
            DynamicValue::Integer(2),
            DynamicValue::Integer(1),
        ]);
        let options = IsDeepEqualOptions {
            strict_order: false,
        };
        assert!(umt_is_deep_equal(&arr1, &arr2, Some(options)));
    }

    #[test]
    fn test_object_comparison() {
        let mut obj1 = HashMap::new();
        obj1.insert("a".to_string(), DynamicValue::Integer(1));
        obj1.insert("b".to_string(), DynamicValue::Integer(2));

        let mut obj2 = HashMap::new();
        obj2.insert("a".to_string(), DynamicValue::Integer(1));
        obj2.insert("b".to_string(), DynamicValue::Integer(2));

        assert!(umt_is_deep_equal(
            &DynamicValue::Object(obj1),
            &DynamicValue::Object(obj2),
            None
        ));
    }

    #[test]
    fn test_object_different_values() {
        let mut obj1 = HashMap::new();
        obj1.insert("a".to_string(), DynamicValue::Integer(1));
        obj1.insert("b".to_string(), DynamicValue::Integer(2));

        let mut obj2 = HashMap::new();
        obj2.insert("a".to_string(), DynamicValue::Integer(1));
        obj2.insert("b".to_string(), DynamicValue::Integer(3));

        assert!(!umt_is_deep_equal(
            &DynamicValue::Object(obj1),
            &DynamicValue::Object(obj2),
            None
        ));
    }

    #[test]
    fn test_object_missing_keys() {
        let mut obj1 = HashMap::new();
        obj1.insert("a".to_string(), DynamicValue::Integer(1));
        obj1.insert("b".to_string(), DynamicValue::Integer(2));

        let mut obj2 = HashMap::new();
        obj2.insert("a".to_string(), DynamicValue::Integer(1));
        obj2.insert("c".to_string(), DynamicValue::Integer(2));

        assert!(!umt_is_deep_equal(
            &DynamicValue::Object(obj1),
            &DynamicValue::Object(obj2),
            None
        ));
    }

    #[test]
    fn test_nested_objects() {
        let mut inner1 = HashMap::new();
        inner1.insert("b".to_string(), DynamicValue::Integer(1));
        let mut obj1 = HashMap::new();
        obj1.insert("a".to_string(), DynamicValue::Object(inner1));

        let mut inner2 = HashMap::new();
        inner2.insert("b".to_string(), DynamicValue::Integer(1));
        let mut obj2 = HashMap::new();
        obj2.insert("a".to_string(), DynamicValue::Object(inner2));

        assert!(umt_is_deep_equal(
            &DynamicValue::Object(obj1),
            &DynamicValue::Object(obj2),
            None
        ));
    }

    #[test]
    fn test_null_comparisons() {
        assert!(umt_is_deep_equal(
            &DynamicValue::Null,
            &DynamicValue::Null,
            None
        ));
        assert!(!umt_is_deep_equal(
            &DynamicValue::Null,
            &DynamicValue::Object(HashMap::new()),
            None
        ));
    }

    #[test]
    fn test_deep_equal_f64() {
        assert!(umt_is_deep_equal_f64(1.0, 1.0));
        assert!(umt_is_deep_equal_f64(f64::NAN, f64::NAN));
        assert!(!umt_is_deep_equal_f64(-0.0, 0.0));
        assert!(!umt_is_deep_equal_f64(1.0, 2.0));
    }

    #[test]
    fn test_deep_equal_generic() {
        assert!(umt_is_deep_equal_generic(&1, &1));
        assert!(umt_is_deep_equal_generic(&"test", &"test"));
        assert!(!umt_is_deep_equal_generic(&1, &2));
    }
}
