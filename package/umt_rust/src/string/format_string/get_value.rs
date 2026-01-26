use crate::internal::json::JsonValue;

/// Retrieves a value from a JSON object using a dot-notation path with array index support.
///
/// Supports nested properties and array access with positive/negative indices.
///
/// # Arguments
///
/// * `object` - The JSON value to retrieve from
/// * `path` - Dot-notation path (e.g., "user.name", "items[0]", "data[0].items[-1].name")
///
/// # Returns
///
/// The value at the specified path, or None if not found
///
/// # Examples
///
/// ```ignore
/// use umt_rust::json;
/// use umt_rust::string::format_string::get_value;
///
/// let obj = json!({"name": "Alice"});
/// assert_eq!(get_value(&obj, "name"), Some(&json!("Alice")));
/// ```
pub fn get_value<'a>(object: &'a JsonValue, path: &str) -> Option<&'a JsonValue> {
    if path.is_empty() {
        return None;
    }

    let parts: Vec<&str> = path.split('.').collect();

    let mut current = object;

    for part in parts {
        // Check if it's an object
        if !current.is_object() && !current.is_array() {
            return None;
        }

        if let Some((key, index)) = parse_array_access(part) {
            // Handle array notation like "items[0]" or "items[-1]"
            // Get the object property first
            current = current.get(key)?;

            // Then get the array element
            if let Some(arr) = current.as_array() {
                let actual_index = if index < 0 {
                    (arr.len() as i64 + index) as usize
                } else {
                    index as usize
                };

                current = arr.get(actual_index)?;
            } else {
                return None;
            }
        } else {
            // Simple property access
            current = current.get(part)?;
        }
    }

    Some(current)
}

fn parse_array_access(s: &str) -> Option<(&str, i64)> {
    let bracket_start = s.find('[')?;
    if !s.ends_with(']') {
        return None;
    }
    let key = &s[..bracket_start];
    let index_str = &s[bracket_start + 1..s.len() - 1];
    let index: i64 = index_str.parse().ok()?;
    if key.is_empty() {
        return None;
    }
    Some((key, index))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json;

    #[test]
    fn test_simple_property_access() {
        let obj = json!({"name": "Alice", "age": 30});

        assert_eq!(get_value(&obj, "name"), Some(&json!("Alice")));
        assert_eq!(get_value(&obj, "age"), Some(&json!(30)));
    }

    #[test]
    fn test_non_existent_properties() {
        let obj = json!({"name": "Alice"});

        assert_eq!(get_value(&obj, "nonexistent"), None);
        assert_eq!(get_value(&obj, "missing"), None);
    }

    #[test]
    fn test_various_data_types() {
        let obj = json!({
            "string": "text",
            "number": 42,
            "boolean": true,
            "nullValue": null,
            "zero": 0,
            "emptyString": ""
        });

        assert_eq!(get_value(&obj, "string"), Some(&json!("text")));
        assert_eq!(get_value(&obj, "number"), Some(&json!(42)));
        assert_eq!(get_value(&obj, "boolean"), Some(&json!(true)));
        assert_eq!(get_value(&obj, "nullValue"), Some(&json!(null)));
        assert_eq!(get_value(&obj, "zero"), Some(&json!(0)));
        assert_eq!(get_value(&obj, "emptyString"), Some(&json!("")));
    }

    #[test]
    fn test_nested_property_access() {
        let obj = json!({
            "user": {
                "name": "Bob",
                "profile": {
                    "age": 25,
                    "location": "Tokyo"
                }
            }
        });

        assert_eq!(get_value(&obj, "user.name"), Some(&json!("Bob")));
        assert_eq!(get_value(&obj, "user.profile.age"), Some(&json!(25)));
        assert_eq!(
            get_value(&obj, "user.profile.location"),
            Some(&json!("Tokyo"))
        );
    }

    #[test]
    fn test_broken_nested_paths() {
        let obj = json!({
            "user": {
                "name": "Bob"
            }
        });

        assert_eq!(get_value(&obj, "user.nonexistent"), None);
        assert_eq!(get_value(&obj, "user.profile.age"), None);
        assert_eq!(get_value(&obj, "nonexistent.property"), None);
    }

    #[test]
    fn test_array_access_positive_indices() {
        let obj = json!({
            "items": ["A", "B", "C", "D"]
        });

        assert_eq!(get_value(&obj, "items[0]"), Some(&json!("A")));
        assert_eq!(get_value(&obj, "items[1]"), Some(&json!("B")));
        assert_eq!(get_value(&obj, "items[2]"), Some(&json!("C")));
        assert_eq!(get_value(&obj, "items[3]"), Some(&json!("D")));
    }

    #[test]
    fn test_array_access_negative_indices() {
        let obj = json!({
            "items": ["A", "B", "C", "D"]
        });

        assert_eq!(get_value(&obj, "items[-1]"), Some(&json!("D")));
        assert_eq!(get_value(&obj, "items[-2]"), Some(&json!("C")));
        assert_eq!(get_value(&obj, "items[-3]"), Some(&json!("B")));
        assert_eq!(get_value(&obj, "items[-4]"), Some(&json!("A")));
    }

    #[test]
    fn test_out_of_bounds_indices() {
        let obj = json!({
            "items": ["A", "B"]
        });

        assert_eq!(get_value(&obj, "items[5]"), None);
        assert_eq!(get_value(&obj, "items[-5]"), None);
    }

    #[test]
    fn test_arrays_of_objects() {
        let obj = json!({
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25},
                {"name": "Charlie", "age": 35}
            ]
        });

        assert_eq!(get_value(&obj, "users[0].name"), Some(&json!("Alice")));
        assert_eq!(get_value(&obj, "users[1].age"), Some(&json!(25)));
        assert_eq!(get_value(&obj, "users[-1].name"), Some(&json!("Charlie")));
    }

    #[test]
    fn test_null_and_primitives() {
        assert_eq!(get_value(&json!(null), "property"), None);
        assert_eq!(get_value(&json!(42), "property"), None);
        assert_eq!(get_value(&json!("string"), "property"), None);
        assert_eq!(get_value(&json!(true), "property"), None);
    }

    #[test]
    fn test_empty_path() {
        let obj = json!({"name": "Alice"});
        assert_eq!(get_value(&obj, ""), None);
    }

    #[test]
    fn test_array_access_on_non_arrays() {
        let obj = json!({
            "notAnArray": "string"
        });

        assert_eq!(get_value(&obj, "notAnArray[0]"), None);
    }
}
