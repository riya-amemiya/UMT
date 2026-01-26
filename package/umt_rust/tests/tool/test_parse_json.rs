use umt_rust::tool::{umt_parse_json, umt_parse_json_value, JsonValue};

#[test]
fn test_should_parse_valid_json_string() {
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
fn test_should_parse_json_value_as_dynamic() {
    let json_string = r#"{"key": "value"}"#;
    let result = umt_parse_json_value(json_string).unwrap();
    assert_eq!(result["key"], "value");
}

#[test]
fn test_should_parse_array_of_numbers() {
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
    let value = result["value"].as_f64().unwrap();
    assert!((value - 3.14159).abs() < 1e-10);
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
    let result = umt_parse_json(json_string).unwrap();
    match result {
        JsonValue::Object(map) => assert!(map.is_empty()),
        _ => panic!("Expected Object"),
    }
}

#[test]
fn test_should_handle_empty_array() {
    let json_string = "[]";
    let result = umt_parse_json(json_string).unwrap();
    match result {
        JsonValue::Array(arr) => assert!(arr.is_empty()),
        _ => panic!("Expected Array"),
    }
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
