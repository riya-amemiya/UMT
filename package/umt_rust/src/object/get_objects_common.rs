use super::Value;
use std::collections::HashMap;

/// Extract key-value pairs common to all input objects.
///
/// A key-value pair is considered common when the key exists in every object
/// and the value is equal across all objects.
/// When all values for a key are objects, the function recurses to find
/// the common subset. If the recursive result is empty, the key is excluded.
///
/// # Arguments
///
/// * `object` - The first object.
/// * `objects` - Additional objects to compare.
///
/// # Returns
///
/// Object containing only the key-value pairs shared by all inputs.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_get_objects_common, Value};
/// use umt_rust::obj;
/// use std::collections::HashMap;
///
/// let obj1 = obj!("a" => 1, "b" => 2);
/// let obj2 = obj!("a" => 1, "c" => 3);
///
/// if let Value::Object(o1) = &obj1 {
///     if let Value::Object(o2) = &obj2 {
///         let result = umt_get_objects_common(o1, &[o2]);
///         assert_eq!(result.get("a"), Some(&Value::Int(1)));
///         assert_eq!(result.get("b"), None);
///     }
/// }
/// ```
pub fn umt_get_objects_common(
    object: &HashMap<String, Value>,
    objects: &[&HashMap<String, Value>],
) -> HashMap<String, Value> {
    if objects.is_empty() {
        return object.clone();
    }

    let mut result = HashMap::new();

    for (key, value) in object {
        let mut is_common = true;
        let mut all_objects = value.is_object();

        for other in objects {
            match other.get(key) {
                None => {
                    is_common = false;
                    break;
                }
                Some(other_value) => {
                    if !other_value.is_object() {
                        all_objects = false;
                    }

                    if !all_objects && other_value != value {
                        is_common = false;
                        break;
                    }
                }
            }
        }

        if !is_common {
            continue;
        }

        if all_objects {
            if let Some(value_obj) = value.as_object() {
                let other_objs: Vec<&HashMap<String, Value>> = objects
                    .iter()
                    .filter_map(|other| other.get(key).and_then(|v| v.as_object()))
                    .collect();

                let nested = umt_get_objects_common(value_obj, &other_objs);

                if !nested.is_empty() {
                    result.insert(key.clone(), Value::Object(nested));
                }
            }
        } else {
            result.insert(key.clone(), value.clone());
        }
    }

    result
}
