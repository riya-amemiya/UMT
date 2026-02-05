use std::collections::HashMap;
use umt_rust::object::{Value, umt_pick_deep};

#[test]
fn test_should_select_simple_keys() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    map.insert("c".to_string(), Value::Int(3));
    let obj = Value::Object(map);

    let result = umt_pick_deep(&obj, &["a", "c"]);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
}

#[test]
fn test_should_select_nested_keys() {
    // Create: { a: { b: { c: 1, d: 2 }, e: 3 }, f: 4 }
    let mut bc = HashMap::new();
    bc.insert("c".to_string(), Value::Int(1));
    bc.insert("d".to_string(), Value::Int(2));

    let mut ab = HashMap::new();
    ab.insert("b".to_string(), Value::Object(bc));
    ab.insert("e".to_string(), Value::Int(3));

    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Object(ab));
    obj.insert("f".to_string(), Value::Int(4));

    let result = umt_pick_deep(&Value::Object(obj.clone()), &["a.b.c", "f"]);

    assert_eq!(result.get("f"), Some(&Value::Int(4)));
    if let Some(Value::Object(a)) = result.get("a") {
        if let Some(Value::Object(b)) = a.get("b") {
            assert_eq!(b.get("c"), Some(&Value::Int(1)));
            assert_eq!(b.get("d"), None); // d should not be picked
        } else {
            panic!("Expected a.b to be an object");
        }
    } else {
        panic!("Expected a to be an object");
    }
}

#[test]
fn test_should_handle_non_existent_keys() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));

    let result = umt_pick_deep(&Value::Object(obj.clone()), &["a", "c"]);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
}

#[test]
fn test_should_handle_no_keys_specified() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));

    let result = umt_pick_deep(&Value::Object(obj.clone()), &[]);

    assert!(result.is_empty());
}

#[test]
fn test_should_handle_objects_containing_arrays() {
    let mut obj = HashMap::new();
    obj.insert(
        "a".to_string(),
        Value::Array(vec![
            Value::Object({
                let mut m = HashMap::new();
                m.insert("b".to_string(), Value::Int(1));
                m
            }),
            Value::Object({
                let mut m = HashMap::new();
                m.insert("c".to_string(), Value::Int(2));
                m
            }),
        ]),
    );
    obj.insert("d".to_string(), Value::Int(3));

    let result = umt_pick_deep(&Value::Object(obj.clone()), &["a", "d"]);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("d"), Some(&Value::Int(3)));
    assert!(result.get("a").is_some());
}

#[test]
fn test_should_handle_objects_with_null_properties() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Null);
    obj.insert("c".to_string(), Value::Int(3));

    let result = umt_pick_deep(&Value::Object(obj.clone()), &["a", "c"]);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Null));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
}

#[test]
fn test_should_handle_deeply_nested_keys() {
    // Create: { a: { b: { c: { d: { e: 5 } } } } }
    let mut e_obj = HashMap::new();
    e_obj.insert("e".to_string(), Value::Int(5));

    let mut d_obj = HashMap::new();
    d_obj.insert("d".to_string(), Value::Object(e_obj));

    let mut c_obj = HashMap::new();
    c_obj.insert("c".to_string(), Value::Object(d_obj));

    let mut b_obj = HashMap::new();
    b_obj.insert("b".to_string(), Value::Object(c_obj));

    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Object(b_obj));

    let result = umt_pick_deep(&Value::Object(obj.clone()), &["a.b.c.d.e"]);

    // Navigate through the result
    if let Some(Value::Object(a)) = result.get("a") {
        if let Some(Value::Object(b)) = a.get("b") {
            if let Some(Value::Object(c)) = b.get("c") {
                if let Some(Value::Object(d)) = c.get("d") {
                    assert_eq!(d.get("e"), Some(&Value::Int(5)));
                } else {
                    panic!("Expected d to be an object");
                }
            } else {
                panic!("Expected c to be an object");
            }
        } else {
            panic!("Expected b to be an object");
        }
    } else {
        panic!("Expected a to be an object");
    }
}

#[test]
fn test_should_handle_keys_that_reference_entire_objects() {
    let mut inner = HashMap::new();
    inner.insert("b".to_string(), Value::Int(1));
    inner.insert("c".to_string(), Value::Int(2));

    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Object(inner.clone()));
    obj.insert("d".to_string(), Value::Int(3));

    let result = umt_pick_deep(&Value::Object(obj.clone()), &["a"]);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get("a"), Some(&Value::Object(inner)));
}

#[test]
fn test_should_handle_keys_without_dots() {
    let mut inner = HashMap::new();
    inner.insert("b".to_string(), Value::Int(1));

    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Object(inner.clone()));
    obj.insert("c".to_string(), Value::Int(2));

    let result = umt_pick_deep(&Value::Object(obj.clone()), &["a", "c"]);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Object(inner)));
    assert_eq!(result.get("c"), Some(&Value::Int(2)));
}

#[test]
fn test_should_handle_empty_string_keys() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));

    let result = umt_pick_deep(&Value::Object(obj.clone()), &[""]);

    assert!(result.is_empty());
}

#[test]
fn test_should_handle_keys_containing_numbers() {
    let mut inner = HashMap::new();
    inner.insert("1".to_string(), Value::Int(1));
    inner.insert("2".to_string(), Value::Int(2));

    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Object(inner));
    obj.insert("b".to_string(), Value::Int(3));

    let result = umt_pick_deep(&Value::Object(obj.clone()), &["a.1"]);

    if let Some(Value::Object(a)) = result.get("a") {
        assert_eq!(a.get("1"), Some(&Value::Int(1)));
    } else {
        panic!("Expected a to be an object");
    }
}

#[test]
fn test_should_handle_duplicate_keys() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));
    obj.insert("c".to_string(), Value::Int(3));

    let result = umt_pick_deep(&Value::Object(obj.clone()), &["a", "a", "c"]);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
}
