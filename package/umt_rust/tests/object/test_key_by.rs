use std::collections::HashMap;
<<<<<<< HEAD
use umt_rust::object::{Value, umt_key_by, umt_key_by_property};
||||||| 30d5753

/// Helper function that mimics keyBy - creates a map keyed by a field from items
fn key_by<T, F>(items: &[T], key_fn: F) -> HashMap<String, T>
where
    T: Clone,
    F: Fn(&T) -> String,
{
    let mut result = HashMap::new();
    for item in items {
        let key = key_fn(item);
        result.insert(key, item.clone());
    }
    result
}

#[derive(Debug, Clone, PartialEq)]
struct User {
    id: String,
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
struct DirectionCode {
    dir: String,
    code: u32,
}
=======
use umt_rust::object::{Value, umt_key_by};
>>>>>>> 36e5fbd009729e51174857904826bd81d5477247

#[test]
<<<<<<< HEAD
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
||||||| 30d5753
fn test_should_create_object_using_property_name_as_key() {
    let input = vec![
        User {
            id: "a1".to_string(),
            name: "Alice".to_string(),
        },
        User {
            id: "b2".to_string(),
            name: "Bob".to_string(),
        },
    ];
=======
fn test_key_by_string() {
    let mut map1 = HashMap::new();
    map1.insert("id".to_string(), Value::String("a".to_string()));
    map1.insert("val".to_string(), Value::Int(1));
>>>>>>> 36e5fbd009729e51174857904826bd81d5477247

<<<<<<< HEAD
    let result = umt_key_by_property(&items, "id");
||||||| 30d5753
    let result = key_by(&input, |u| u.id.clone());
=======
    let mut map2 = HashMap::new();
    map2.insert("id".to_string(), Value::String("b".to_string()));
    map2.insert("val".to_string(), Value::Int(2));
>>>>>>> 36e5fbd009729e51174857904826bd81d5477247

<<<<<<< HEAD
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
||||||| 30d5753
    assert_eq!(result.len(), 2);
    assert_eq!(
        result.get("a1"),
        Some(&User {
            id: "a1".to_string(),
            name: "Alice".to_string()
        })
    );
    assert_eq!(
        result.get("b2"),
        Some(&User {
            id: "b2".to_string(),
            name: "Bob".to_string()
        })
    );
}

#[test]
fn test_should_generate_keys_using_custom_function() {
    let input = vec![
        DirectionCode {
            dir: "left".to_string(),
            code: 97,
        },
        DirectionCode {
            dir: "right".to_string(),
            code: 100,
        },
    ];

    let result = key_by(&input, |o| char::from_u32(o.code).unwrap().to_string());

    assert_eq!(result.len(), 2);
    assert_eq!(
        result.get("a"),
        Some(&DirectionCode {
            dir: "left".to_string(),
            code: 97
        })
    );
    assert_eq!(
        result.get("d"),
        Some(&DirectionCode {
            dir: "right".to_string(),
            code: 100
        })
    );
}

#[test]
fn test_should_return_empty_map_for_empty_array() {
    let input: Vec<User> = vec![];
    let result = key_by(&input, |u| u.id.clone());
    assert!(result.is_empty());
}

#[test]
fn test_should_use_later_values_when_duplicate_keys() {
    let input = vec![
        User {
            id: "a1".to_string(),
            name: "Alice".to_string(),
        },
        User {
            id: "a1".to_string(),
            name: "Alex".to_string(),
        },
    ];

    let result = key_by(&input, |u| u.id.clone());

    assert_eq!(result.len(), 1);
    assert_eq!(
        result.get("a1"),
        Some(&User {
            id: "a1".to_string(),
            name: "Alex".to_string()
        })
    );
}

#[test]
fn test_should_act_as_identity_function_when_key_is_value() {
    let input = vec!["a".to_string(), "b".to_string()];
    let result = key_by(&input, |s| s.clone());

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&"a".to_string()));
    assert_eq!(result.get("b"), Some(&"b".to_string()));
=======
    let items = Value::Array(vec![Value::Object(map1), Value::Object(map2)]);

    let result = umt_key_by(&items, "id");

    assert!(result.get("a").is_some());
    assert!(result.get("b").is_some());
>>>>>>> 36e5fbd009729e51174857904826bd81d5477247
}
