use std::collections::HashMap;
use umt_rust::object::*;

#[test]
fn test_value_creation() {
    let null = Value::Null;
    assert!(null.is_null());

    let bool_val = Value::Bool(true);
    assert!(!bool_val.is_object());

    let int_val = Value::Int(42);
    assert_eq!(int_val, Value::Int(42));

    let float_val = Value::Float(3.14);
    assert_eq!(float_val, Value::Float(3.14));

    let string_val = Value::String("hello".to_string());
    assert_eq!(string_val, Value::String("hello".to_string()));
}

#[test]
fn test_value_object() {
    let mut map = HashMap::new();
    map.insert("key".to_string(), Value::Int(123));
    let obj = Value::Object(map);

    assert!(obj.is_object());
    assert!(obj.as_object().is_some());
}

#[test]
fn test_value_conversions() {
    let bool_val: Value = true.into();
    assert_eq!(bool_val, Value::Bool(true));

    let int_val: Value = 42i32.into();
    assert_eq!(int_val, Value::Int(42));

    let string_val: Value = "hello".into();
    assert_eq!(string_val, Value::String("hello".to_string()));
}
