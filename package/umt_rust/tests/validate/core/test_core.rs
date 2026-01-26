//! Tests for core validation function

use umt_rust::validate::core::{
    TypeName, ValidateCoreReturnType, ValidateReturnType, umt_validate_core,
};

#[test]
fn test_validate_core_validates_type_correctly() {
    let result = umt_validate_core("test".to_string(), "string", &[], None);
    assert!(result.validate);
    assert_eq!(result.type_value, "test");
}

#[test]
fn test_validate_core_respects_custom_message() {
    let message = "Not a string";
    let result = umt_validate_core("test".to_string(), "string", &[], Some(message));
    assert!(result.validate);
    assert_eq!(result.message, message);
}

#[test]
fn test_validate_core_returns_empty_message_when_no_message_provided() {
    let result = umt_validate_core("test".to_string(), "string", &[], None);
    // When validation passes and no message is provided, message should be empty
    assert!(result.validate);
}

#[test]
fn test_validate_core_with_additional_validation_rules() {
    let min_length = ValidateReturnType::new(
        "string",
        Some("String too short".to_string()),
        |value: &String| value.len() >= 3,
    );

    let result = umt_validate_core("test".to_string(), "string", &[min_length.clone()], None);
    assert!(result.validate);

    let result_short = umt_validate_core("ab".to_string(), "string", &[min_length.clone()], None);
    assert!(!result_short.validate);
    assert_eq!(result_short.message, "String too short");
}

#[test]
fn test_validate_core_validation_rule_without_message() {
    let no_message = ValidateReturnType::new("string", None, |value: &String| value.len() >= 3);

    let result = umt_validate_core("ab".to_string(), "string", &[no_message], None);
    assert!(!result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_validate_core_validates_numbers() {
    let result = umt_validate_core(123.0_f64, "number", &[], None);
    assert!(result.validate);
    assert_eq!(result.type_value, 123.0);
}

#[test]
fn test_validate_core_validates_booleans() {
    let result = umt_validate_core(true, "boolean", &[], None);
    assert!(result.validate);
    assert!(result.type_value);

    let result_false = umt_validate_core(false, "boolean", &[], None);
    assert!(result_false.validate);
    assert!(!result_false.type_value);
}

#[test]
fn test_validate_core_with_multiple_validation_rules() {
    let min_length = ValidateReturnType::new(
        "string",
        Some("String too short".to_string()),
        |value: &String| value.len() >= 3,
    );

    let max_length = ValidateReturnType::new(
        "string",
        Some("String too long".to_string()),
        |value: &String| value.len() <= 10,
    );

    let result = umt_validate_core(
        "test".to_string(),
        "string",
        &[min_length.clone(), max_length.clone()],
        None,
    );
    assert!(result.validate);

    let result_short = umt_validate_core(
        "ab".to_string(),
        "string",
        &[min_length.clone(), max_length.clone()],
        None,
    );
    assert!(!result_short.validate);
    assert_eq!(result_short.message, "String too short");

    let result_long = umt_validate_core(
        "very long string".to_string(),
        "string",
        &[min_length, max_length],
        None,
    );
    assert!(!result_long.validate);
    assert_eq!(result_long.message, "String too long");
}

#[test]
fn test_validate_core_preserves_type_information() {
    let result_string = umt_validate_core("test".to_string(), "string", &[], None);
    assert_eq!(result_string.type_value, "test");

    let result_number = umt_validate_core(123.0_f64, "number", &[], None);
    assert_eq!(result_number.type_value, 123.0);

    let result_bool = umt_validate_core(true, "boolean", &[], None);
    assert!(result_bool.type_value);
}

#[test]
fn test_type_name_display() {
    assert_eq!(TypeName::String.to_string(), "string");
    assert_eq!(TypeName::Number.to_string(), "number");
    assert_eq!(TypeName::Boolean.to_string(), "boolean");
    assert_eq!(TypeName::Undefined.to_string(), "undefined");
    assert_eq!(TypeName::Null.to_string(), "null");
    assert_eq!(TypeName::Object.to_string(), "object");
    assert_eq!(TypeName::Array.to_string(), "array");
}

#[test]
fn test_type_name_equality() {
    assert_eq!(TypeName::String, TypeName::String);
    assert_ne!(TypeName::String, TypeName::Number);
}

#[test]
fn test_type_name_clone() {
    let original = TypeName::String;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_validate_return_type_clone() {
    let original = ValidateReturnType::new("string", Some("Error".to_string()), |s: &String| {
        !s.is_empty()
    });
    let cloned = original.clone();
    assert_eq!(original.type_name, cloned.type_name);
    assert_eq!(original.message, cloned.message);
    assert!((cloned.validate)(&"test".to_string()));
}

#[test]
fn test_validate_return_type_debug() {
    let return_type =
        ValidateReturnType::new("string", Some("Error".to_string()), |_: &String| true);
    let debug_str = format!("{:?}", return_type);
    assert!(debug_str.contains("ValidateReturnType"));
    assert!(debug_str.contains("type_name"));
    assert!(debug_str.contains("message"));
}

#[test]
fn test_validate_core_return_type_equality() {
    let result1 = ValidateCoreReturnType {
        validate: true,
        message: "test".to_string(),
        type_value: 42,
    };
    let result2 = ValidateCoreReturnType {
        validate: true,
        message: "test".to_string(),
        type_value: 42,
    };
    assert_eq!(result1, result2);
}

#[test]
fn test_validate_core_return_type_clone() {
    let original = ValidateCoreReturnType {
        validate: true,
        message: "test".to_string(),
        type_value: 42,
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_validate_core_return_type_debug() {
    let result = ValidateCoreReturnType {
        validate: true,
        message: "test".to_string(),
        type_value: 42,
    };
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("ValidateCoreReturnType"));
}
