use std::collections::HashMap;
use umt_rust::obj;
use umt_rust::object::{Value, umt_remove_prototype};

fn as_map(value: Value) -> HashMap<String, Value> {
    value.into_object().unwrap()
}

#[test]
fn test_should_remove_prototype_polluting_properties() {
    let object = as_map(obj! {
        "__proto__" => obj!{"polluted" => true},
        "constructor" => obj!{"prototype" => obj!{"polluted" => true}},
        "prototype" => obj!{"polluted" => true},
        "a" => 1
    });

    let result = umt_remove_prototype(&object);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("__proto__"), None);
    assert_eq!(result.get("constructor"), None);
    assert_eq!(result.get("prototype"), None);
}

#[test]
fn test_should_preserve_other_properties() {
    let object = as_map(obj! {
        "a" => 1,
        "b" => "test",
        "c" => true,
        "d" => Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
        "e" => obj!{"nested" => true}
    });

    let result = umt_remove_prototype(&object);
    assert_eq!(result, object);
}

#[test]
fn test_should_not_modify_the_original_object() {
    let object = as_map(obj! {"__proto__" => obj!{"polluted" => true}, "a" => 1});
    let snapshot = object.clone();

    let result = umt_remove_prototype(&object);

    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(object, snapshot);
}

#[test]
fn test_should_perform_a_shallow_copy() {
    // Nested dangerous keys are preserved (shallow operation).
    let object = as_map(obj! {
        "safe" => obj!{"__proto__" => obj!{"polluted" => true}, "value" => 1}
    });

    let result = umt_remove_prototype(&object);

    let safe = result.get("safe").and_then(Value::as_object).unwrap();
    assert!(safe.contains_key("__proto__"));
    assert_eq!(safe.get("value"), Some(&Value::Int(1)));
}

#[test]
fn test_should_handle_an_empty_object() {
    let object: HashMap<String, Value> = HashMap::new();
    let result = umt_remove_prototype(&object);
    assert!(result.is_empty());
}

#[test]
fn test_should_handle_dangerous_names_mixed_with_safe_ones() {
    let object = as_map(obj! {
        "__proto__" => 1,
        "a" => 2,
        "constructor" => 3,
        "b" => 4,
        "prototype" => 5,
        "c" => 6
    });

    let result = umt_remove_prototype(&object);

    let mut expected = HashMap::new();
    expected.insert("a".to_string(), Value::Int(2));
    expected.insert("b".to_string(), Value::Int(4));
    expected.insert("c".to_string(), Value::Int(6));
    assert_eq!(result, expected);
}
