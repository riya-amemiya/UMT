use super::Value;

/// Recursively merges own and inherited enumerable string keyed properties of source objects into the destination object.
///
/// # Arguments
///
/// * `object` - The destination object
/// * `sources` - The source objects
///
/// # Returns
///
/// Returns a new object with deeply merged properties.
pub fn umt_merge_deep(object: &Value, sources: &[Value]) -> Value {
    let mut current_target = object.clone();

    for source in sources {
        current_target = merge_deep_two(&current_target, source);
    }

    current_target
}

fn merge_deep_two(target: &Value, source: &Value) -> Value {
    match (target, source) {
        (Value::Object(target_map), Value::Object(source_map)) => {
            let mut result_map = target_map.clone();

            for (key, source_value) in source_map {
                if let Some(target_value) = result_map.get(key) {
                    let merged_value = merge_deep_two(target_value, source_value);
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
