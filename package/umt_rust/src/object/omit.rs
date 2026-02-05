use super::Value;

/// Creates an object composed of the own and inherited enumerable property paths of object that are not omitted.
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
            let mut result_map = map.clone();
            for path in paths {
                result_map.remove(*path);
            }
            Value::Object(result_map)
        }
        _ => object.clone(),
    }
}
