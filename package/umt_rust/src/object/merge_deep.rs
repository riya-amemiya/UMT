use super::Value;

/// Recursively merges own and inherited enumerable string keyed properties of source objects into the destination object.
///
/// # Arguments
///
/// * `object` - The destination object
/// * `source` - The source object
///
/// # Returns
///
/// Returns a new object with deeply merged properties.
pub fn umt_merge_deep(object: &Value, source: &Value) -> Value {
    match (object, source) {
        (Value::Object(target_map), Value::Object(source_map)) => {
            let mut result_map = target_map.clone();

            for (key, source_value) in source_map {
                if let Some(target_value) = result_map.get(key) {
                    let merged_value = umt_merge_deep(target_value, source_value);
                    result_map.insert(key.clone(), merged_value);
                } else {
                    result_map.insert(key.clone(), source_value.clone());
                }
            }

            Value::Object(result_map)
        }
        (_, _) => source.clone(),
    }
}
