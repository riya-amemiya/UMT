use super::Value;
use std::collections::HashMap;

/// Creates an object composed of the picked object properties, supporting deep paths.
///
/// # Arguments
///
/// * `object` - The source object
/// * `paths` - The property paths to pick (dot notation supported)
///
/// # Returns
///
/// Returns the new object.
pub fn umt_pick_deep(object: &Value, paths: &[&str]) -> Value {
    match object {
        Value::Object(_) => {
            let mut result_map = HashMap::new();

            for path in paths {
                if let Some(val) = get_deep(object, path) {
                    insert_deep(&mut result_map, path, val);
                }
            }

            Value::Object(result_map)
        }
        _ => Value::Object(HashMap::new()),
    }
}

fn get_deep(object: &Value, path: &str) -> Option<Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = object;

    for part in parts {
        match current {
            Value::Object(map) => {
                if let Some(val) = map.get(part) {
                    current = val;
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    Some(current.clone())
}

fn insert_deep(map: &mut HashMap<String, Value>, path: &str, value: Value) {
    let parts: Vec<&str> = path.split('.').collect();
    if parts.is_empty() {
        return;
    }

    if parts.len() == 1 {
        map.insert(parts[0].to_string(), value);
        return;
    }

    let key = parts[0];
    let rest = parts[1..].join(".");

    let entry = map
        .entry(key.to_string())
        .or_insert_with(|| Value::Object(HashMap::new()));

    if let Value::Object(inner_map) = entry {
        insert_deep(inner_map, &rest, value);
    }
}
