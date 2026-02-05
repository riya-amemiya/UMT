use crate::object::Value;
use regex::Regex;
use std::sync::OnceLock;

static ARRAY_PATTERN: OnceLock<Regex> = OnceLock::new();

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

    let array_pattern = ARRAY_PATTERN.get_or_init(|| Regex::new(r"^(.+?)\[(-?\d+)\]$").unwrap());
    let parts: Vec<&str> = path.split('.').collect();

    let mut current = object;

    for part in parts {
        // Check if it's an object or array (if index access is attempted)
        // Actually, if it's an array access like "items[0]", `part` is "items[0]".
        // So we need to handle that.

        if let Some(caps) = array_pattern.captures(part) {
            // Handle array notation like "items[0]" or "items[-1]"
            let key = caps.get(1)?.as_str();
            let index: i64 = caps.get(2)?.as_str().parse().ok()?;

            // Get the object property first
            current = current.get(key)?;

            // Then get the array element
            if let Some(arr) = current.as_array() {
                let actual_index = if index < 0 {
                    let abs_index = index.unsigned_abs() as usize;
                    if abs_index > arr.len() {
                         return None;
                    }
                    arr.len() - abs_index
                } else {
                    index as usize
                };

                if actual_index >= arr.len() {
                    return None;
                }

                current = &arr[actual_index];
            } else {
                return None;
            }
        } else {
            // Simple property access
             if !current.is_object() {
                return None;
            }
            current = current.get(part)?;
        }
    }

    Some(current)
}
