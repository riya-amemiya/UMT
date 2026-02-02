use super::Value;
use std::collections::HashMap;

/// Checks if an object is empty (has no own properties).
///
/// # Arguments
///
/// * `object` - The object to check
///
/// # Returns
///
/// Returns true if the object is empty, false otherwise.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_is_empty, Value};
/// use std::collections::HashMap;
///
/// let obj = HashMap::new();
/// assert!(umt_is_empty(&obj));
/// ```
pub fn umt_is_empty(object: &HashMap<String, Value>) -> bool {
    object.is_empty()
}
