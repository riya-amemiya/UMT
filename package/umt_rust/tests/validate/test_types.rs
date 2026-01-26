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
