//! Object pattern matching predicate

use std::collections::HashMap;

use serde_json::Value;

/// Creates a predicate that checks whether a JSON object matches
/// all properties of the given pattern using strict equality
///
/// # Arguments
/// * `pattern` - The pattern to match against
///
/// # Returns
/// A predicate that tests objects against the pattern
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use serde_json::Value;
/// use umt_rust::predicate::umt_matches;
///
/// let mut pattern = HashMap::new();
/// pattern.insert("role".to_string(), Value::String("admin".to_string()));
/// let is_admin = umt_matches(pattern);
///
/// let mut obj = HashMap::new();
/// obj.insert("name".to_string(), Value::String("Alice".to_string()));
/// obj.insert("role".to_string(), Value::String("admin".to_string()));
/// assert!(is_admin(&obj));
/// ```
#[inline]
pub fn umt_matches(pattern: HashMap<String, Value>) -> impl Fn(&HashMap<String, Value>) -> bool {
    move |object: &HashMap<String, Value>| {
        for (key, pattern_value) in &pattern {
            match object.get(key) {
                Some(obj_value) if obj_value == pattern_value => {}
                _ => return false,
            }
        }
        true
    }
}
