use serde::de::DeserializeOwned;
use serde_json;

/// Error type for JSON parsing failures
#[derive(Debug, Clone, PartialEq)]
pub struct JsonParseError {
    pub message: String,
}

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for JsonParseError {}

impl From<serde_json::Error> for JsonParseError {
    fn from(err: serde_json::Error) -> Self {
        JsonParseError {
            message: err.to_string(),
        }
    }
}

/// Parses a JSON string into a typed Rust value.
///
/// # Arguments
///
/// * `json` - A JSON string to parse
///
/// # Returns
///
/// Returns `Ok(T)` with the parsed value on success,
/// or `Err(JsonParseError)` if parsing fails.
///
/// # Example
///
/// ```
/// use umt_rust::tool::umt_parse_json;
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug, PartialEq)]
/// struct Person {
///     name: String,
///     age: i32,
/// }
///
/// let json = r#"{"name": "John", "age": 30}"#;
/// let person: Person = umt_parse_json(json).unwrap();
/// assert_eq!(person.name, "John");
/// assert_eq!(person.age, 30);
/// ```
#[inline]
pub fn umt_parse_json<T: DeserializeOwned>(json: &str) -> Result<T, JsonParseError> {
    serde_json::from_str(json).map_err(JsonParseError::from)
}

/// Parses a JSON string into a serde_json::Value.
///
/// This is useful when you don't know the structure of the JSON beforehand.
///
/// # Arguments
///
/// * `json` - A JSON string to parse
///
/// # Returns
///
/// Returns `Ok(serde_json::Value)` on success,
/// or `Err(JsonParseError)` if parsing fails.
///
/// # Example
///
/// ```
/// use umt_rust::tool::umt_parse_json_value;
///
/// let json = r#"{"key": "value"}"#;
/// let value = umt_parse_json_value(json).unwrap();
/// assert_eq!(value["key"], "value");
/// ```
#[inline]
pub fn umt_parse_json_value(json: &str) -> Result<serde_json::Value, JsonParseError> {
    umt_parse_json(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::collections::HashMap;

    #[test]
    fn test_should_parse_valid_json_string() {
        let json_string = r#"{"key": "value"}"#;
        let result: HashMap<String, String> = umt_parse_json(json_string).unwrap();
        let mut expected = HashMap::new();
        expected.insert("key".to_string(), "value".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_should_return_error_for_invalid_json_string() {
        let invalid_json_string = r#"{"key": "value""#;
        let result: Result<serde_json::Value, _> = umt_parse_json(invalid_json_string);
        assert!(result.is_err());
    }

    #[test]
    fn test_should_parse_json_string_with_array() {
        let json_string = r#"["value1", "value2"]"#;
        let result: Vec<String> = umt_parse_json(json_string).unwrap();
        assert_eq!(result, vec!["value1".to_string(), "value2".to_string()]);
    }

    #[test]
    fn test_should_parse_json_string_with_nested_objects() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Nested {
            #[serde(rename = "nestedKey")]
            nested_key: String,
        }

        #[derive(Deserialize, Debug, PartialEq)]
        struct Outer {
            key: Nested,
        }

        let json_string = r#"{"key": {"nestedKey": "nestedValue"}}"#;
        let result: Outer = umt_parse_json(json_string).unwrap();
        assert_eq!(result, Outer {
            key: Nested {
                nested_key: "nestedValue".to_string(),
            },
        });
    }

    #[test]
    fn test_should_parse_json_string_with_numbers() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Data {
            key: i32,
        }

        let json_string = r#"{"key": 123}"#;
        let result: Data = umt_parse_json(json_string).unwrap();
        assert_eq!(result, Data { key: 123 });
    }

    #[test]
    fn test_should_parse_json_string_with_boolean_values() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Data {
            key: bool,
        }

        let json_string = r#"{"key": true}"#;
        let result: Data = umt_parse_json(json_string).unwrap();
        assert_eq!(result, Data { key: true });
    }

    #[test]
    fn test_should_parse_json_value() {
        let json_string = r#"{"key": "value"}"#;
        let result = umt_parse_json_value(json_string).unwrap();
        assert_eq!(result["key"], "value");
    }

    #[test]
    fn test_should_parse_json_array_of_numbers() {
        let json_string = r#"[1, 2, 3]"#;
        let result: Vec<i32> = umt_parse_json(json_string).unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_should_parse_null_value() {
        let json_string = "null";
        let result: Option<i32> = umt_parse_json(json_string).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_should_parse_floating_point_numbers() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Data {
            value: f64,
        }

        let json_string = r#"{"value": 3.14159}"#;
        let result: Data = umt_parse_json(json_string).unwrap();
        assert!((result.value - 3.14159).abs() < 1e-10);
    }
}
