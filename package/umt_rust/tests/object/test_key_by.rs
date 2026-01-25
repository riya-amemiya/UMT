use std::collections::HashMap;

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

#[test]
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

    let result = key_by(&input, |u| u.id.clone());

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
}
