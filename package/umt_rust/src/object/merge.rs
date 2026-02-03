use super::Value;

/// Merges own enumerable string keyed properties of source object to destination object.
/// This is a shallow merge.
///
/// # Arguments
///
/// * `object` - The destination object
/// * `source` - The source object
///
/// # Returns
///
/// Returns a new object with merged properties.
pub fn umt_merge(object: &Value, source: &Value) -> Value {
    let mut result_map = match object {
        Value::Object(map) => map.clone(),
        _ => return object.clone(),
    };

    if let Value::Object(source_map) = source {
        for (k, v) in source_map {
            result_map.insert(k.clone(), v.clone());
        }
    }

    Value::Object(result_map)
}
