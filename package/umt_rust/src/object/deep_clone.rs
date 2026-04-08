//! Deep clone utility for Value objects.
//!
//! Provides a recursive deep clone that filters out dangerous keys
//! (`__proto__`, `constructor`, `prototype`) to match the behavior
//! of the TypeScript source implementation.

use super::Value;
use std::collections::HashMap;

const MAX_CLONE_DEPTH: usize = 100;

/// Recursively clones a Value, filtering dangerous keys from objects.
fn clone_value(value: &Value, depth: usize) -> Value {
    if depth > MAX_CLONE_DEPTH {
        panic!(
            "deep_clone: maximum recursion depth of {} exceeded",
            MAX_CLONE_DEPTH
        );
    }

    match value {
        Value::Object(map) => {
            let mut result = HashMap::new();
            for (key, val) in map {
                if key == "__proto__" || key == "constructor" || key == "prototype" {
                    continue;
                }
                result.insert(key.clone(), clone_value(val, depth + 1));
            }
            Value::Object(result)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(|v| clone_value(v, depth + 1)).collect()),
        other => other.clone(),
    }
}

/// Creates a deep clone of a Value.
///
/// Objects are recursively cloned with dangerous keys (`__proto__`,
/// `constructor`, `prototype`) filtered out to prevent prototype pollution
/// when interoperating with JavaScript runtimes.
///
/// # Arguments
///
/// * `value` - The Value to deep clone
///
/// # Returns
///
/// A new Value that is a deep copy of the input
///
/// # Panics
///
/// Panics if recursion depth exceeds 100, indicating a circular-like structure.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_deep_clone, Value};
/// use std::collections::HashMap;
///
/// let mut inner = HashMap::new();
/// inner.insert("c".to_string(), Value::Int(2));
///
/// let mut original = HashMap::new();
/// original.insert("a".to_string(), Value::Int(1));
/// original.insert("b".to_string(), Value::Object(inner));
///
/// let cloned = umt_deep_clone(&Value::Object(original));
///
/// // Modifying original does not affect clone
/// if let Value::Object(map) = &cloned {
///     assert_eq!(map.get("a"), Some(&Value::Int(1)));
/// }
/// ```
#[inline]
pub fn umt_deep_clone(value: &Value) -> Value {
    clone_value(value, 0)
}
