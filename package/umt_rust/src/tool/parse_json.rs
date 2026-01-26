use crate::internal::json::{JsonParseError, JsonValue, parse_json};

/// Parses a JSON string into a JsonValue.
///
/// # Arguments
///
/// * `json` - A JSON string to parse
///
/// # Returns
///
/// Returns `Ok(JsonValue)` with the parsed value on success,
/// or `Err(JsonParseError)` if parsing fails.
///
/// # Example
///
/// ```
/// use umt_rust::tool::umt_parse_json;
///
/// let value = umt_parse_json(r#"{"name": "John", "age": 30}"#).unwrap();
/// assert_eq!(value["name"].as_str(), Some("John"));
/// assert_eq!(value["age"].as_f64(), Some(30.0));
/// ```
#[inline]
pub fn umt_parse_json(json_str: &str) -> Result<JsonValue, JsonParseError> {
    parse_json(json_str)
}

/// Alias for umt_parse_json - parses JSON string to JsonValue.
///
/// # Example
///
/// ```
/// use umt_rust::tool::umt_parse_json_value;
///
/// let value = umt_parse_json_value(r#"{"key": "value"}"#).unwrap();
/// assert_eq!(value["key"], "value");
/// ```
#[inline]
pub fn umt_parse_json_value(json_str: &str) -> Result<JsonValue, JsonParseError> {
    umt_parse_json(json_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_parse_valid_json_object() {
        let json_string = r#"{"key": "value"}"#;
        let result = umt_parse_json(json_string).unwrap();
        assert_eq!(result["key"], "value");
    }

    #[test]
    fn test_should_return_error_for_invalid_json_string() {
        let invalid_json_string = r#"{"key": "value""#;
        let result = umt_parse_json(invalid_json_string);
        assert!(result.is_err());
    }

    #[test]
    fn test_should_parse_json_string_with_array() {
        let json_string = r#"["value1", "value2"]"#;
        let result = umt_parse_json(json_string).unwrap();
        assert_eq!(result[0], "value1");
        assert_eq!(result[1], "value2");
    }

    #[test]
    fn test_should_parse_json_string_with_nested_objects() {
        let json_string = r#"{"key": {"nestedKey": "nestedValue"}}"#;
        let result = umt_parse_json(json_string).unwrap();
        assert_eq!(result["key"]["nestedKey"], "nestedValue");
    }

    #[test]
    fn test_should_parse_json_string_with_numbers() {
        let json_string = r#"{"key": 123}"#;
        let result = umt_parse_json(json_string).unwrap();
        assert_eq!(result["key"], 123);
    }

    #[test]
    fn test_should_parse_json_string_with_boolean_values() {
        let json_string = r#"{"key": true}"#;
        let result = umt_parse_json(json_string).unwrap();
        assert_eq!(result["key"], true);
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
        let result = umt_parse_json(json_string).unwrap();
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 2);
        assert_eq!(result[2], 3);
    }

    #[test]
    fn test_should_parse_null_value() {
        let json_string = "null";
        let result = umt_parse_json(json_string).unwrap();
        assert!(result.is_null());
    }

    #[test]
    fn test_should_parse_floating_point_numbers() {
        let json_string = r#"{"value": 3.14159}"#;
        let result = umt_parse_json(json_string).unwrap();
        let val = result["value"].as_f64().unwrap();
        assert!((val - 3.14159).abs() < 1e-10);
    }
}
