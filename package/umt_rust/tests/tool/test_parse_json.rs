use serde::Deserialize;
use std::collections::HashMap;
use umt_rust::tool::{umt_parse_json, umt_parse_json_value};

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
    assert_eq!(
        result,
        Outer {
            key: Nested {
                nested_key: "nestedValue".to_string(),
            },
        }
    );
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
fn test_should_parse_json_value_as_dynamic() {
    let json_string = r#"{"key": "value"}"#;
    let result = umt_parse_json_value(json_string).unwrap();
    assert_eq!(result["key"], "value");
}

#[test]
fn test_should_parse_array_of_numbers() {
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

#[test]
fn test_should_parse_complex_nested_structure() {
    let json_string = r#"{
        "users": [
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25}
        ],
        "total": 2
    }"#;

    let result = umt_parse_json_value(json_string).unwrap();
    assert_eq!(result["total"], 2);
    assert_eq!(result["users"][0]["name"], "Alice");
    assert_eq!(result["users"][1]["age"], 25);
}

#[test]
fn test_should_handle_empty_object() {
    let json_string = "{}";
    let result: HashMap<String, String> = umt_parse_json(json_string).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_should_handle_empty_array() {
    let json_string = "[]";
    let result: Vec<i32> = umt_parse_json(json_string).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_should_handle_string_with_special_characters() {
    let json_string = r#"{"key": "hello\nworld\ttab"}"#;
    let result = umt_parse_json_value(json_string).unwrap();
    assert_eq!(result["key"], "hello\nworld\ttab");
}

#[test]
fn test_should_handle_unicode_strings() {
    let json_string = r#"{"key": "hello \u4e16\u754c"}"#;
    let result = umt_parse_json_value(json_string).unwrap();
    assert!(result["key"].as_str().unwrap().contains("hello"));
}

#[test]
fn test_json_parse_error_display() {
    use umt_rust::tool::JsonParseError;

    let error = JsonParseError {
        message: "test error".to_string(),
    };
    assert_eq!(format!("{}", error), "test error");
}

#[test]
fn test_json_parse_error_from_serde_error() {
    let result: Result<serde_json::Value, _> = umt_parse_json("invalid json");
    let error = result.unwrap_err();
    assert!(!error.message.is_empty());
}
