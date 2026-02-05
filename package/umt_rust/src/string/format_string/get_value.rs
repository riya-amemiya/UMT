use crate::object::Value;
use regex::Regex;

/// Retrieves a value from a JSON object using a dot-notation path with array index support.
///
/// Supports nested properties and array access with positive/negative indices.
///
/// # Arguments
///
/// * `object` - The JSON value to retrieve from
/// * `path` - Dot-notation path (e.g., "user.name", "items[0]", "data[0].items[-1].name")
///
/// # Returns
///
/// The value at the specified path, or None if not found
///
/// # Examples
///
/// ```ignore
/// use umt_rust::object::Value;
/// use umt_rust::string::format_string::get_value;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("name".to_string(), Value::String("Alice".to_string()));
/// let obj = Value::Object(map);
/// assert_eq!(get_value(&obj, "name"), Some(&Value::String("Alice".to_string())));
/// ```
pub fn get_value<'a>(object: &'a Value, path: &str) -> Option<&'a Value> {
    if path.is_empty() {
        return None;
    }

    let array_pattern = Regex::new(r"^(.+?)\[(-?\d+)\]$").unwrap();
    let parts: Vec<&str> = path.split('.').collect();

    let mut current = object;

    for part in parts {
        // Check if it's an object
        if !current.is_object() && !current.is_array() {
            return None;
        }

        if let Some(caps) = array_pattern.captures(part) {
            // Handle array notation like "items[0]" or "items[-1]"
            let key = caps.get(1)?.as_str();
            let index: i64 = caps.get(2)?.as_str().parse().ok()?;

            // Get the object property first
            current = current.get(key)?;

            // Then get the array element
            if let Some(arr) = current.as_array() {
                let actual_index = if index < 0 {
                    (arr.len() as i64 + index) as usize
                } else {
                    index as usize
                };

                current = arr.get(actual_index)?;
            } else {
                return None;
            }
        } else {
            // Simple property access
            current = current.get(part)?;
        }
    }

    Some(current)
}
