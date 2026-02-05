use super::Value;
use std::collections::HashMap;

/// Creates an object composed of the picked object properties.
///
/// # Arguments
///
/// * `object` - The source object
/// * `paths` - The property paths to pick
///
/// # Returns
///
/// Returns the new object.
pub fn umt_pick(object: &Value, paths: &[&str]) -> Value {
    match object {
        Value::Object(map) => {
            let mut result_map = HashMap::new();
            for path in paths {
                if let Some(val) = map.get(*path) {
                    result_map.insert(path.to_string(), val.clone());
                }
            }
            Value::Object(result_map)
        }
        _ => Value::Object(HashMap::new()),
    }
}
