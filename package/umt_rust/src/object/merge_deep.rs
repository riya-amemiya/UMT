<<<<<<< HEAD
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
||||||| 55b8153
=======
use super::Value;
use std::collections::HashMap;

/// Deeply merges multiple objects into a single object.
///
/// # Arguments
///
/// * `target` - The target object to merge into
/// * `sources` - The source objects to merge from
///
/// # Returns
///
/// A new deeply merged HashMap.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_merge_deep, Value};
/// use std::collections::HashMap;
///
/// let mut inner_target = HashMap::new();
/// inner_target.insert("b".to_string(), Value::Int(1));
/// inner_target.insert("c".to_string(), Value::Int(2));
///
/// let mut target = HashMap::new();
/// target.insert("a".to_string(), Value::Object(inner_target));
///
/// let mut inner_source = HashMap::new();
/// inner_source.insert("c".to_string(), Value::Int(3));
/// inner_source.insert("d".to_string(), Value::Int(4));
///
/// let mut source = HashMap::new();
/// source.insert("a".to_string(), Value::Object(inner_source));
///
/// let result = umt_merge_deep(&target, &[&source]);
///
/// if let Some(Value::Object(a)) = result.get("a") {
///     assert_eq!(a.get("b"), Some(&Value::Int(1))); // preserved from target
///     assert_eq!(a.get("c"), Some(&Value::Int(3))); // overwritten by source
///     assert_eq!(a.get("d"), Some(&Value::Int(4))); // added from source
/// } else {
///     panic!("Expected object for key 'a'");
/// }
/// ```
pub fn umt_merge_deep(
    target: &HashMap<String, Value>,
    sources: &[&HashMap<String, Value>],
) -> HashMap<String, Value> {
    let mut result = target.clone();

    for source in sources {
        result = merge_deep_impl(result, (*source).clone());
    }

    result
}

/// Deeply merges two HashMaps.
fn merge_deep_impl(
    target: HashMap<String, Value>,
    source: HashMap<String, Value>,
) -> HashMap<String, Value> {
    let mut result = target;

    for (key, source_value) in source {
        let target_value = result.get(&key);

        let new_value = match (target_value, &source_value) {
            (Some(Value::Object(target_obj)), Value::Object(source_obj)) => {
                // Both are objects, merge recursively
                Value::Object(merge_deep_impl(target_obj.clone(), source_obj.clone()))
            }
            _ => {
                // Otherwise, use the source value
                source_value
            }
        };

        result.insert(key, new_value);
    }

    result
}

/// Deeply merges two objects into a single object.
///
/// # Arguments
///
/// * `target` - The target object to merge into
/// * `source` - The source object to merge from
///
/// # Returns
///
/// A new deeply merged HashMap.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_merge_deep_two, Value};
/// use std::collections::HashMap;
///
/// let mut inner_target = HashMap::new();
/// inner_target.insert("x".to_string(), Value::Int(1));
///
/// let mut target = HashMap::new();
/// target.insert("nested".to_string(), Value::Object(inner_target));
///
/// let mut inner_source = HashMap::new();
/// inner_source.insert("y".to_string(), Value::Int(2));
///
/// let mut source = HashMap::new();
/// source.insert("nested".to_string(), Value::Object(inner_source));
///
/// let result = umt_merge_deep_two(&target, &source);
///
/// if let Some(Value::Object(nested)) = result.get("nested") {
///     assert_eq!(nested.get("x"), Some(&Value::Int(1)));
///     assert_eq!(nested.get("y"), Some(&Value::Int(2)));
/// } else {
///     panic!("Expected object for key 'nested'");
/// }
/// ```
pub fn umt_merge_deep_two(
    target: &HashMap<String, Value>,
    source: &HashMap<String, Value>,
) -> HashMap<String, Value> {
    umt_merge_deep(target, &[source])
}
>>>>>>> d916e13f83cf5d012a5cda2956ae6890fbe99788
