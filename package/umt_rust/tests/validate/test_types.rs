use std::collections::HashMap;
use umt_rust::validate::types::*;

#[test]
fn test_value_type_display() {
    assert_eq!(ValueType::String.to_string(), "string");
    assert_eq!(ValueType::Number.to_string(), "number");
    assert_eq!(ValueType::Boolean.to_string(), "boolean");
    assert_eq!(ValueType::Null.to_string(), "null");
    assert_eq!(ValueType::Undefined.to_string(), "undefined");
    assert_eq!(ValueType::Array.to_string(), "array");
    assert_eq!(ValueType::Object.to_string(), "object");
}

#[test]
fn test_validate_core_result_success() {
    let result = ValidateCoreResult::success(42, ValueType::Number);
    assert!(result.validate);
    assert_eq!(result.message, "");
    assert_eq!(result.value_type, ValueType::Number);
    assert_eq!(result.value, Some(42));
}

#[test]
fn test_validate_core_result_failure() {
    let result: ValidateCoreResult<i32> =
        ValidateCoreResult::failure("Invalid value", ValueType::Number);
    assert!(!result.validate);
    assert_eq!(result.message, "Invalid value");
    assert_eq!(result.value_type, ValueType::Number);
    assert_eq!(result.value, None);
}

#[test]
fn test_dynamic_value_type_detection() {
    assert_eq!(DynamicValue::Null.value_type(), ValueType::Null);
    assert_eq!(DynamicValue::Bool(true).value_type(), ValueType::Boolean);
    assert_eq!(DynamicValue::Integer(42).value_type(), ValueType::Number);
    assert_eq!(DynamicValue::Float(3.14).value_type(), ValueType::Number);
    assert_eq!(
        DynamicValue::String("test".to_string()).value_type(),
        ValueType::String
    );
    assert_eq!(DynamicValue::Array(vec![]).value_type(), ValueType::Array);
    assert_eq!(
        DynamicValue::Object(HashMap::new()).value_type(),
        ValueType::Object
    );
}

#[test]
fn test_dynamic_value_type_checks() {
    let null = DynamicValue::Null;
    assert!(null.is_null());
    assert!(!null.is_bool());

    let bool_val = DynamicValue::Bool(true);
    assert!(bool_val.is_bool());
    assert!(!bool_val.is_number());

    let int_val = DynamicValue::Integer(42);
    assert!(int_val.is_number());
    assert!(!int_val.is_string());

    let float_val = DynamicValue::Float(3.14);
    assert!(float_val.is_number());

    let string_val = DynamicValue::String("test".to_string());
    assert!(string_val.is_string());
    assert!(!string_val.is_array());

    let array_val = DynamicValue::Array(vec![]);
    assert!(array_val.is_array());
    assert!(!array_val.is_object());

    let object_val = DynamicValue::Object(HashMap::new());
    assert!(object_val.is_object());
    assert!(!object_val.is_null());
}

#[test]
fn test_dynamic_value_conversions() {
    assert_eq!(DynamicValue::Bool(true).as_bool(), Some(true));
    assert_eq!(DynamicValue::Integer(42).as_i64(), Some(42));
    assert_eq!(DynamicValue::Float(3.14).as_f64(), Some(3.14));
    assert_eq!(
        DynamicValue::String("test".to_string()).as_str(),
        Some("test")
    );

    let arr = vec![DynamicValue::Integer(1), DynamicValue::Integer(2)];
    let array_val = DynamicValue::Array(arr.clone());
    assert_eq!(array_val.as_array(), Some(&arr));

    let mut obj = HashMap::new();
    obj.insert("key".to_string(), DynamicValue::Integer(42));
    let object_val = DynamicValue::Object(obj.clone());
    assert_eq!(object_val.as_object(), Some(&obj));
}

#[test]
fn test_validate_core_result_failure_optional() {
    // With message
    let result: ValidateCoreResult<i32> =
        ValidateCoreResult::failure_optional(Some("Error message"), ValueType::String);
    assert!(!result.validate);
    assert_eq!(result.message, "Error message");
    assert_eq!(result.value_type, ValueType::String);
    assert_eq!(result.value, None);

    // Without message
    let result2: ValidateCoreResult<i32> =
        ValidateCoreResult::failure_optional(None, ValueType::Boolean);
    assert!(!result2.validate);
    assert_eq!(result2.message, "");
    assert_eq!(result2.value_type, ValueType::Boolean);
    assert_eq!(result2.value, None);
}

#[test]
fn test_validate_return_type_new() {
    let return_type = ValidateReturnType::new(
        ValueType::Number,
        |n: &i32| *n > 0,
        Some("Must be positive".to_string()),
    );
    assert_eq!(return_type.value_type, ValueType::Number);
    assert_eq!(return_type.message, Some("Must be positive".to_string()));
    assert!((return_type.validate)(&5));
    assert!(!(return_type.validate)(&-1));

    // Without message
    let return_type2 = ValidateReturnType::new(ValueType::String, |s: &String| !s.is_empty(), None);
    assert_eq!(return_type2.value_type, ValueType::String);
    assert_eq!(return_type2.message, None);
    assert!((return_type2.validate)(&"hello".to_string()));
    assert!(!(return_type2.validate)(&"".to_string()));
}

#[test]
fn test_dynamic_value_conversions_none_cases() {
    // as_bool returns None for non-bool
    assert_eq!(DynamicValue::Integer(42).as_bool(), None);
    assert_eq!(DynamicValue::Null.as_bool(), None);

    // as_i64 returns None for non-numeric (except float converts)
    assert_eq!(DynamicValue::String("test".to_string()).as_i64(), None);
    assert_eq!(DynamicValue::Bool(true).as_i64(), None);
    // Float to i64 conversion
    assert_eq!(DynamicValue::Float(3.7).as_i64(), Some(3));

    // as_f64 returns None for non-numeric
    assert_eq!(DynamicValue::String("test".to_string()).as_f64(), None);
    assert_eq!(DynamicValue::Bool(true).as_f64(), None);
    // Integer to f64 conversion
    assert_eq!(DynamicValue::Integer(42).as_f64(), Some(42.0));

    // as_str returns None for non-string
    assert_eq!(DynamicValue::Integer(42).as_str(), None);
    assert_eq!(DynamicValue::Bool(true).as_str(), None);

    // as_array returns None for non-array
    assert_eq!(DynamicValue::Integer(42).as_array(), None);
    assert_eq!(DynamicValue::String("test".to_string()).as_array(), None);

    // as_object returns None for non-object
    assert_eq!(DynamicValue::Integer(42).as_object(), None);
    assert_eq!(DynamicValue::Array(vec![]).as_object(), None);
}

#[test]
fn test_value_type_equality() {
    assert_eq!(ValueType::String, ValueType::String);
    assert_ne!(ValueType::String, ValueType::Number);
}

#[test]
fn test_dynamic_value_clone() {
    let original = DynamicValue::Integer(42);
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_dynamic_value_equality() {
    assert_eq!(DynamicValue::Integer(42), DynamicValue::Integer(42));
    assert_ne!(DynamicValue::Integer(42), DynamicValue::Integer(43));
    assert_ne!(DynamicValue::Integer(42), DynamicValue::Float(42.0));
}
