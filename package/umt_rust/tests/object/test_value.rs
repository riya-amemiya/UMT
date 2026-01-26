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

#[test]
fn test_value_as_object_mut() {
    let mut map = HashMap::new();
    map.insert("key".to_string(), Value::Int(123));
    let mut obj = Value::Object(map);

    if let Some(inner_map) = obj.as_object_mut() {
        inner_map.insert("new_key".to_string(), Value::Bool(true));
    }
    assert_eq!(obj.as_object().unwrap().len(), 2);

    // Non-object returns None
    let mut int_val = Value::Int(42);
    assert!(int_val.as_object_mut().is_none());
}

#[test]
fn test_value_into_object() {
    let mut map = HashMap::new();
    map.insert("key".to_string(), Value::Int(123));
    let obj = Value::Object(map);

    let inner_map = obj.into_object();
    assert!(inner_map.is_some());
    assert_eq!(inner_map.unwrap().len(), 1);

    // Non-object returns None
    let int_val = Value::Int(42);
    assert!(int_val.into_object().is_none());
}

#[test]
fn test_value_display() {
    assert_eq!(format!("{}", Value::Null), "null");
    assert_eq!(format!("{}", Value::Bool(true)), "true");
    assert_eq!(format!("{}", Value::Bool(false)), "false");
    assert_eq!(format!("{}", Value::Int(42)), "42");
    assert_eq!(format!("{}", Value::Float(3.14)), "3.14");
    assert_eq!(
        format!("{}", Value::String("hello".to_string())),
        "\"hello\""
    );

    // Array display
    let arr = Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
    assert_eq!(format!("{}", arr), "[1, 2, 3]");

    // Empty array
    let empty_arr = Value::Array(vec![]);
    assert_eq!(format!("{}", empty_arr), "[]");

    // Object display (note: HashMap order is not guaranteed)
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    let obj = Value::Object(map);
    let display = format!("{}", obj);
    assert!(display.contains("\"a\": 1"));

    // Empty object
    let empty_obj = Value::Object(HashMap::new());
    assert_eq!(format!("{}", empty_obj), "{}");
}

#[test]
fn test_value_from_i64() {
    let val: Value = 123i64.into();
    assert_eq!(val, Value::Int(123));
}

#[test]
fn test_value_from_f64() {
    let val: Value = 3.14f64.into();
    assert_eq!(val, Value::Float(3.14));
}

#[test]
fn test_value_from_string() {
    let val: Value = String::from("hello").into();
    assert_eq!(val, Value::String("hello".to_string()));
}

#[test]
fn test_value_from_vec() {
    let vec: Vec<i32> = vec![1, 2, 3];
    let val: Value = vec.into();
    assert_eq!(
        val,
        Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
    );
}

#[test]
fn test_value_from_hashmap() {
    let mut map = HashMap::new();
    map.insert("key".to_string(), Value::Int(123));
    let val: Value = map.clone().into();
    assert_eq!(val, Value::Object(map));
}

#[test]
fn test_value_from_option() {
    let some_val: Value = Some(42i32).into();
    assert_eq!(some_val, Value::Int(42));

    let none_val: Value = None::<i32>.into();
    assert_eq!(none_val, Value::Null);
}

#[test]
fn test_value_default() {
    let default_val = Value::default();
    assert_eq!(default_val, Value::Null);
}

#[test]
fn test_value_clone() {
    let original = Value::Int(42);
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_value_array() {
    let arr = Value::Array(vec![Value::Int(1), Value::Bool(true)]);
    assert!(!arr.is_object());
    assert!(!arr.is_null());
}
