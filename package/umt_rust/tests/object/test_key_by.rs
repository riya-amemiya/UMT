use std::collections::HashMap;
use umt_rust::object::{umt_key_by, umt_key_by_property, Value};

#[test]
fn test_should_create_object_using_property_name_as_key() {
    let items = vec![
        Value::Object({
            let mut m = HashMap::new();
            m.insert("id".to_string(), Value::String("a1".to_string()));
            m.insert("name".to_string(), Value::String("Alice".to_string()));
            m
        }),
        Value::Object({
            let mut m = HashMap::new();
            m.insert("id".to_string(), Value::String("b2".to_string()));
            m.insert("name".to_string(), Value::String("Bob".to_string()));
            m
        }),
    ];

    let result = umt_key_by_property(&items, "id");

    assert_eq!(result.len(), 2);
    assert!(result.contains_key("a1"));
    assert!(result.contains_key("b2"));

    if let Some(Value::Object(alice)) = result.get("a1") {
        assert_eq!(alice.get("name"), Some(&Value::String("Alice".to_string())));
    } else {
        panic!("Expected object for key 'a1'");
    }
}

#[test]
fn test_should_generate_keys_using_custom_function() {
    let items = vec![
        Value::Object({
            let mut m = HashMap::new();
            m.insert("dir".to_string(), Value::String("left".to_string()));
            m.insert("code".to_string(), Value::Int(97));
            m
        }),
        Value::Object({
            let mut m = HashMap::new();
            m.insert("dir".to_string(), Value::String("right".to_string()));
            m.insert("code".to_string(), Value::Int(100));
            m
        }),
    ];

    let result = umt_key_by(&items, |v| {
        if let Value::Object(obj) = v {
            if let Some(Value::Int(code)) = obj.get("code") {
                return char::from_u32(*code as u32)
                    .map(|c| c.to_string())
                    .unwrap_or_default();
            }
        }
        String::new()
    });

    assert_eq!(result.len(), 2);
    assert!(result.contains_key("a")); // char code 97 = 'a'
    assert!(result.contains_key("d")); // char code 100 = 'd'
}

#[test]
fn test_should_return_empty_map_for_empty_array() {
    let items: Vec<Value> = vec![];
    let result = umt_key_by(&items, |v| {
        if let Value::String(s) = v {
            s.clone()
        } else {
            String::new()
        }
    });
    assert!(result.is_empty());
}

#[test]
fn test_should_use_later_values_when_duplicate_keys() {
    let items = vec![
        Value::Object({
            let mut m = HashMap::new();
            m.insert("id".to_string(), Value::String("a1".to_string()));
            m.insert("name".to_string(), Value::String("Alice".to_string()));
            m
        }),
        Value::Object({
            let mut m = HashMap::new();
            m.insert("id".to_string(), Value::String("a1".to_string()));
            m.insert("name".to_string(), Value::String("Alex".to_string()));
            m
        }),
    ];

    let result = umt_key_by_property(&items, "id");

    assert_eq!(result.len(), 1);
    assert!(result.contains_key("a1"));

    if let Some(Value::Object(obj)) = result.get("a1") {
        assert_eq!(obj.get("name"), Some(&Value::String("Alex".to_string())));
    } else {
        panic!("Expected object for key 'a1'");
    }
}

#[test]
fn test_should_act_as_identity_function_when_key_is_value() {
    let items = vec![
        Value::String("a".to_string()),
        Value::String("b".to_string()),
    ];

    let result = umt_key_by(&items, |v| {
        if let Value::String(s) = v {
            s.clone()
        } else {
            String::new()
        }
    });

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::String("a".to_string())));
    assert_eq!(result.get("b"), Some(&Value::String("b".to_string())));
}

#[test]
fn test_key_by_with_int_values() {
    let items = vec![
        Value::Object({
            let mut m = HashMap::new();
            m.insert("id".to_string(), Value::Int(1));
            m.insert("value".to_string(), Value::String("one".to_string()));
            m
        }),
        Value::Object({
            let mut m = HashMap::new();
            m.insert("id".to_string(), Value::Int(2));
            m.insert("value".to_string(), Value::String("two".to_string()));
            m
        }),
    ];

    let result = umt_key_by_property(&items, "id");

    assert_eq!(result.len(), 2);
    assert!(result.contains_key("1"));
    assert!(result.contains_key("2"));
}
