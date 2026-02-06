use super::Value;
use std::collections::HashMap;
use std::collections::HashSet;

/// Extract key-value pairs that appear in exactly one input object.
///
/// A key-value pair is considered "shared" if the same key with the same value
/// exists in two or more objects. Only pairs unique to a single object are returned.
/// When all values for a key are objects, the function recurses to find
/// the diff subset. If the recursive result is empty, the key is excluded.
/// When multiple unique pairs share the same key (different values),
/// the last value wins.
///
/// # Arguments
///
/// * `object` - The first object.
/// * `objects` - Additional objects to compare.
///
/// # Returns
///
/// Object containing only key-value pairs unique to one input.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_get_objects_diff, Value};
/// use umt_rust::obj;
/// use std::collections::HashMap;
///
/// let obj1 = obj!("a" => 1, "b" => 2);
/// let obj2 = obj!("b" => 2, "c" => 3);
///
/// if let Value::Object(o1) = &obj1 {
///     if let Value::Object(o2) = &obj2 {
///         let result = umt_get_objects_diff(o1, &[o2]);
///         assert_eq!(result.get("a"), Some(&Value::Int(1)));
///         assert_eq!(result.get("c"), Some(&Value::Int(3)));
///         assert_eq!(result.get("b"), None);
///     }
/// }
/// ```
pub fn umt_get_objects_diff(
    object: &HashMap<String, Value>,
    objects: &[&HashMap<String, Value>],
) -> HashMap<String, Value> {
    let all_objects: Vec<&HashMap<String, Value>> = {
        let mut v = vec![object];
        v.extend_from_slice(objects);
        v
    };

    if all_objects.len() == 1 {
        return object.clone();
    }

    let mut all_keys = HashSet::new();
    for obj in &all_objects {
        for key in obj.keys() {
            all_keys.insert(key.clone());
        }
    }

    let mut result = HashMap::new();

    for key in &all_keys {
        let mut values: Vec<&Value> = Vec::new();

        for obj in &all_objects {
            if let Some(v) = obj.get(key) {
                values.push(v);
            }
        }

        if values.len() == 1 {
            result.insert(key.clone(), values[0].clone());
            continue;
        }

        let all_plain = values.iter().all(|v| v.is_object());

        if all_plain {
            let obj_values: Vec<&HashMap<String, Value>> =
                values.iter().filter_map(|v| v.as_object()).collect();

            if obj_values.len() >= 2 {
                let nested = umt_get_objects_diff(obj_values[0], &obj_values[1..]);

                if !nested.is_empty() {
                    result.insert(key.clone(), Value::Object(nested));
                }
            }
            continue;
        }

        let mut last_unique_value: Option<&Value> = None;

        for i in 0..values.len() {
            let mut count = 0;
            for j in 0..values.len() {
                if values[i] == values[j] {
                    count += 1;
                }
            }
            if count == 1 {
                last_unique_value = Some(values[i]);
            }
        }

        if let Some(v) = last_unique_value {
            result.insert(key.clone(), v.clone());
        }
    }

    result
}
