use std::collections::HashMap;
use umt_rust::object::{Value, umt_get_objects_common};

// Ported from package/main/src/tests/unit/Object/getObjectsCommon.test.ts.
// JS-only cases (prototype pollution, NaN reference equality, array-vs-plain-
// object shape confusion, primitive type coercion) are omitted because the
// Rust `Value` enum is strongly typed and cannot reproduce them.

#[test]
fn test_should_find_common_key_value_pairs_between_two_objects() {
    let obj1 = HashMap::from([
        ("a".to_string(), Value::Int(1)),
        ("b".to_string(), Value::Int(2)),
    ]);
    let obj2 = HashMap::from([
        ("a".to_string(), Value::Int(1)),
        ("c".to_string(), Value::Int(3)),
    ]);

    let result = umt_get_objects_common(&obj1, &[&obj2]);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
}

#[test]
fn test_should_find_common_key_value_pairs_among_three_objects() {
    let obj1 = HashMap::from([
        ("a".to_string(), Value::Int(1)),
        ("b".to_string(), Value::Int(2)),
        ("c".to_string(), Value::Int(3)),
    ]);
    let obj2 = HashMap::from([
        ("a".to_string(), Value::Int(1)),
        ("b".to_string(), Value::Int(2)),
        ("d".to_string(), Value::Int(4)),
    ]);
    let obj3 = HashMap::from([
        ("a".to_string(), Value::Int(1)),
        ("e".to_string(), Value::Int(5)),
    ]);

    let result = umt_get_objects_common(&obj1, &[&obj2, &obj3]);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
}

#[test]
fn test_should_return_a_shallow_copy_for_a_single_object() {
    let obj = HashMap::from([
        ("a".to_string(), Value::Int(1)),
        ("b".to_string(), Value::Int(2)),
    ]);

    let mut result = umt_get_objects_common(&obj, &[]);

    assert_eq!(result, obj);
    // The returned HashMap is owned, so mutating it must not touch the input.
    result.insert("__probe__".to_string(), Value::Int(999));
    assert!(!obj.contains_key("__probe__"));
}

#[test]
fn test_should_return_empty_object_when_one_input_is_empty() {
    let obj = HashMap::from([
        ("a".to_string(), Value::Int(1)),
        ("b".to_string(), Value::Int(2)),
    ]);
    let empty: HashMap<String, Value> = HashMap::new();

    assert!(umt_get_objects_common(&obj, &[&empty]).is_empty());
    assert!(umt_get_objects_common(&empty, &[&obj]).is_empty());
}

#[test]
fn test_should_return_empty_object_when_all_inputs_are_empty() {
    let a: HashMap<String, Value> = HashMap::new();
    let b: HashMap<String, Value> = HashMap::new();

    assert!(umt_get_objects_common(&a, &[&b]).is_empty());
}

#[test]
fn test_should_return_empty_object_when_no_keys_are_common() {
    let obj1 = HashMap::from([("a".to_string(), Value::Int(1))]);
    let obj2 = HashMap::from([("b".to_string(), Value::Int(2))]);

    assert!(umt_get_objects_common(&obj1, &[&obj2]).is_empty());
}

#[test]
fn test_should_exclude_keys_with_different_values() {
    let obj1 = HashMap::from([
        ("a".to_string(), Value::Int(1)),
        ("b".to_string(), Value::Int(2)),
    ]);
    let obj2 = HashMap::from([
        ("a".to_string(), Value::Int(1)),
        ("b".to_string(), Value::Int(5)),
    ]);

    let result = umt_get_objects_common(&obj1, &[&obj2]);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
}

#[test]
fn test_should_handle_falsy_values_correctly() {
    let make = || {
        HashMap::from([
            ("a".to_string(), Value::Null),
            ("b".to_string(), Value::Bool(false)),
            ("c".to_string(), Value::Int(0)),
            ("d".to_string(), Value::Float(0.0)),
            ("e".to_string(), Value::String(String::new())),
        ])
    };
    let obj1 = make();
    let obj2 = make();

    let result = umt_get_objects_common(&obj1, &[&obj2]);

    assert_eq!(result, obj1);
}

#[test]
fn test_should_handle_falsy_values_that_differ() {
    let obj_null = HashMap::from([("a".to_string(), Value::Null)]);
    let obj_zero = HashMap::from([("a".to_string(), Value::Int(0))]);
    let obj_false = HashMap::from([("a".to_string(), Value::Bool(false))]);
    let obj_empty = HashMap::from([("a".to_string(), Value::String(String::new()))]);

    assert!(umt_get_objects_common(&obj_null, &[&obj_zero]).is_empty());
    assert!(umt_get_objects_common(&obj_zero, &[&obj_false]).is_empty());
    assert!(umt_get_objects_common(&obj_empty, &[&obj_zero]).is_empty());
}

#[test]
fn test_should_find_common_nested_objects_recursively() {
    let inner1 = HashMap::from([
        ("b".to_string(), Value::Int(1)),
        ("c".to_string(), Value::Int(2)),
    ]);
    let inner2 = HashMap::from([
        ("b".to_string(), Value::Int(1)),
        ("d".to_string(), Value::Int(4)),
    ]);
    let obj1 = HashMap::from([
        ("a".to_string(), Value::Object(inner1)),
        ("d".to_string(), Value::Int(3)),
    ]);
    let obj2 = HashMap::from([
        ("a".to_string(), Value::Object(inner2)),
        ("d".to_string(), Value::Int(3)),
    ]);

    let result = umt_get_objects_common(&obj1, &[&obj2]);

    let expected_inner = HashMap::from([("b".to_string(), Value::Int(1))]);
    assert_eq!(result.get("a"), Some(&Value::Object(expected_inner)));
    assert_eq!(result.get("d"), Some(&Value::Int(3)));
    assert_eq!(result.len(), 2);
}

#[test]
fn test_should_exclude_key_when_recursive_result_is_empty() {
    let obj1 = HashMap::from([(
        "a".to_string(),
        Value::Object(HashMap::from([("b".to_string(), Value::Int(1))])),
    )]);
    let obj2 = HashMap::from([(
        "a".to_string(),
        Value::Object(HashMap::from([("c".to_string(), Value::Int(2))])),
    )]);

    assert!(umt_get_objects_common(&obj1, &[&obj2]).is_empty());
}

#[test]
fn test_should_handle_deeply_nested_objects() {
    let deep1 = HashMap::from([
        ("d".to_string(), Value::Int(1)),
        ("e".to_string(), Value::Int(2)),
    ]);
    let deep2 = HashMap::from([
        ("d".to_string(), Value::Int(1)),
        ("f".to_string(), Value::Int(3)),
    ]);
    let obj1 = HashMap::from([(
        "a".to_string(),
        Value::Object(HashMap::from([(
            "b".to_string(),
            Value::Object(HashMap::from([("c".to_string(), Value::Object(deep1))])),
        )])),
    )]);
    let obj2 = HashMap::from([(
        "a".to_string(),
        Value::Object(HashMap::from([(
            "b".to_string(),
            Value::Object(HashMap::from([("c".to_string(), Value::Object(deep2))])),
        )])),
    )]);

    let result = umt_get_objects_common(&obj1, &[&obj2]);

    let expected = HashMap::from([(
        "a".to_string(),
        Value::Object(HashMap::from([(
            "b".to_string(),
            Value::Object(HashMap::from([(
                "c".to_string(),
                Value::Object(HashMap::from([("d".to_string(), Value::Int(1))])),
            )])),
        )])),
    )]);
    assert_eq!(result, expected);
}

#[test]
fn test_should_handle_nested_objects_among_three_objects() {
    let inner1 = HashMap::from([
        ("b".to_string(), Value::Int(1)),
        ("c".to_string(), Value::Int(2)),
        ("d".to_string(), Value::Int(3)),
    ]);
    let inner2 = HashMap::from([
        ("b".to_string(), Value::Int(1)),
        ("c".to_string(), Value::Int(2)),
        ("e".to_string(), Value::Int(4)),
    ]);
    let inner3 = HashMap::from([
        ("b".to_string(), Value::Int(1)),
        ("f".to_string(), Value::Int(5)),
    ]);
    let obj1 = HashMap::from([("a".to_string(), Value::Object(inner1))]);
    let obj2 = HashMap::from([("a".to_string(), Value::Object(inner2))]);
    let obj3 = HashMap::from([("a".to_string(), Value::Object(inner3))]);

    let result = umt_get_objects_common(&obj1, &[&obj2, &obj3]);

    let expected = HashMap::from([(
        "a".to_string(),
        Value::Object(HashMap::from([("b".to_string(), Value::Int(1))])),
    )]);
    assert_eq!(result, expected);
}

#[test]
fn test_should_handle_mixed_nested_and_primitive_values() {
    let obj1 = HashMap::from([(
        "a".to_string(),
        Value::Object(HashMap::from([("b".to_string(), Value::Int(1))])),
    )]);
    let obj2 = HashMap::from([("a".to_string(), Value::String("hello".to_string()))]);

    assert!(umt_get_objects_common(&obj1, &[&obj2]).is_empty());
}

#[test]
fn test_should_not_mutate_original_objects() {
    let obj1 = HashMap::from([
        ("a".to_string(), Value::Int(1)),
        (
            "b".to_string(),
            Value::Object(HashMap::from([("c".to_string(), Value::Int(2))])),
        ),
    ]);
    let obj2 = HashMap::from([
        ("a".to_string(), Value::Int(1)),
        (
            "b".to_string(),
            Value::Object(HashMap::from([
                ("c".to_string(), Value::Int(2)),
                ("d".to_string(), Value::Int(3)),
            ])),
        ),
    ]);

    let obj1_snapshot = obj1.clone();
    let obj2_snapshot = obj2.clone();

    let _ = umt_get_objects_common(&obj1, &[&obj2]);

    assert_eq!(obj1, obj1_snapshot);
    assert_eq!(obj2, obj2_snapshot);
}

#[test]
fn test_should_return_an_empty_result_when_either_object_lacks_a_key_present_in_the_other() {
    let obj1 = HashMap::from([
        ("a".to_string(), Value::Int(1)),
        ("b".to_string(), Value::Int(2)),
    ]);
    let obj2 = HashMap::from([("a".to_string(), Value::Int(1))]);

    // Only `a` is iterated from obj2 (right side drives the iteration when used
    // as the first arg), so the intersection yields `{ a: 1 }` regardless of
    // obj1 having an extra `b`.
    let left = umt_get_objects_common(&obj1, &[&obj2]);
    assert_eq!(left.len(), 1);
    assert_eq!(left.get("a"), Some(&Value::Int(1)));

    let right = umt_get_objects_common(&obj2, &[&obj1]);
    assert_eq!(right.len(), 1);
    assert_eq!(right.get("a"), Some(&Value::Int(1)));
}

#[test]
fn test_should_handle_many_objects_efficiently() {
    let objects: Vec<HashMap<String, Value>> = (0..20)
        .map(|i| {
            HashMap::from([
                ("shared".to_string(), Value::Int(1)),
                ("unique".to_string(), Value::Int(i)),
            ])
        })
        .collect();

    let rest: Vec<&HashMap<String, Value>> = objects[1..].iter().collect();
    let result = umt_get_objects_common(&objects[0], &rest);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get("shared"), Some(&Value::Int(1)));
}
