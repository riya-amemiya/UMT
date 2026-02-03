use super::Value;
use std::collections::HashMap;

/// Creates an object composed of keys generated from the results of running each element of collection through iteratee.
///
/// # Arguments
///
/// * `collection` - The collection to iterate over (Array or Object)
/// * `iteratee` - The path to the property to use as the key
///
/// # Returns
///
/// Returns a new Object with keys mapped to values.
pub fn umt_key_by(collection: &Value, iteratee: &str) -> Value {
    let mut result = HashMap::new();

    match collection {
        Value::Array(arr) => {
            for item in arr {
                let key = extract_key(item, iteratee);
                result.insert(key, item.clone());
            }
        }
        Value::Object(obj) => {
            for (_, item) in obj {
                let key = extract_key(item, iteratee);
                result.insert(key, item.clone());
            }
        }
        _ => {}
    }

    Value::Object(result)
}

fn extract_key(item: &Value, path: &str) -> String {
    if path.is_empty() {
        return item.to_string();
    }

    // Simple property access for now, could be expanded to dot notation if needed
    match item {
        Value::Object(map) => {
            if let Some(val) = map.get(path) {
                match val {
                    Value::String(s) => s.clone(),
                    v => v.to_string(),
                }
            } else {
                "undefined".to_string()
            }
        }
        _ => "undefined".to_string(),
    }
}
