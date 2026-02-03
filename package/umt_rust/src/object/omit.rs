use super::Value;
use std::collections::HashMap;

/// The opposite of `pick`; this method creates an object composed of the own and inherited enumerable property paths of object that are not omitted.
///
/// # Arguments
///
/// * `object` - The source object
/// * `paths` - The property paths to omit
///
/// # Returns
///
/// Returns the new object.
pub fn umt_omit(object: &Value, paths: &[&str]) -> Value {
    match object {
        Value::Object(map) => {
            let mut result_map = HashMap::new();
            for (k, v) in map {
                if !paths.contains(&k.as_str()) {
                    result_map.insert(k.clone(), v.clone());
                }
            }
            Value::Object(result_map)
        }
        _ => object.clone(),
    }
}
