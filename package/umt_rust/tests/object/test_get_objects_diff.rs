use std::collections::HashMap;
use umt_rust::object::{Value, umt_get_objects_diff};

fn object_of(entries: Vec<(&str, Value)>) -> HashMap<String, Value> {
    entries
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect()
}

#[test]
fn test_should_find_diff_key_value_pairs_between_two_objects() {
    let obj1 = object_of(vec![("a", Value::Int(1)), ("b", Value::Int(2))]);
    let obj2 = object_of(vec![("b", Value::Int(2)), ("c", Value::Int(3))]);

    let result = umt_get_objects_diff(&obj1, &[&obj2]);

    assert_eq!(
        result,
        object_of(vec![("a", Value::Int(1)), ("c", Value::Int(3))])
    );
}

#[test]
fn test_should_find_diff_key_value_pairs_among_three_objects() {
    let obj1 = object_of(vec![
        ("a", Value::Int(1)),
        ("b", Value::Int(2)),
        ("c", Value::Int(3)),
    ]);
    let obj2 = object_of(vec![
        ("b", Value::Int(2)),
        ("c", Value::Int(3)),
        ("d", Value::Int(4)),
    ]);
    let obj3 = object_of(vec![
        ("c", Value::Int(3)),
        ("d", Value::Int(4)),
        ("e", Value::Int(5)),
    ]);

    let result = umt_get_objects_diff(&obj1, &[&obj2, &obj3]);

    assert_eq!(
        result,
        object_of(vec![("a", Value::Int(1)), ("e", Value::Int(5))])
    );
}

#[test]
fn test_should_return_a_copy_for_a_single_object() {
    let obj = object_of(vec![("a", Value::Int(1)), ("b", Value::Int(2))]);

    assert_eq!(umt_get_objects_diff(&obj, &[]), obj);
}

#[test]
fn test_should_return_empty_object_when_objects_are_identical() {
    let obj1 = object_of(vec![("a", Value::Int(1)), ("b", Value::Null)]);
    let obj2 = obj1.clone();

    assert!(umt_get_objects_diff(&obj1, &[&obj2]).is_empty());
}

#[test]
fn test_should_handle_empty_objects() {
    let empty = HashMap::new();
    let obj = object_of(vec![("a", Value::Int(1))]);

    assert_eq!(umt_get_objects_diff(&empty, &[&obj]), obj);
    assert_eq!(umt_get_objects_diff(&obj, &[&empty]), obj);
    assert!(umt_get_objects_diff(&empty, &[&empty]).is_empty());
}

#[test]
fn test_should_use_last_value_when_same_key_has_different_unique_values() {
    let obj1 = object_of(vec![("a", Value::Int(1))]);
    let obj2 = object_of(vec![("a", Value::Int(2))]);

    assert_eq!(umt_get_objects_diff(&obj1, &[&obj2]), obj2);
}

#[test]
fn test_should_handle_shared_and_unique_values_across_three_objects() {
    let one = object_of(vec![("x", Value::Int(1))]);
    let two = object_of(vec![("x", Value::Int(2))]);
    let three = object_of(vec![("x", Value::Int(3))]);

    assert_eq!(umt_get_objects_diff(&one, &[&one, &two]), two);
    assert_eq!(umt_get_objects_diff(&three, &[&one, &one]), three);
    assert!(umt_get_objects_diff(&one, &[&one, &one]).is_empty());
}

#[test]
fn test_should_find_diff_in_nested_objects_recursively() {
    let obj1 = object_of(vec![
        (
            "a",
            Value::Object(object_of(vec![("b", Value::Int(1)), ("c", Value::Int(2))])),
        ),
        ("d", Value::Int(3)),
    ]);
    let obj2 = object_of(vec![
        (
            "a",
            Value::Object(object_of(vec![("b", Value::Int(1)), ("d", Value::Int(4))])),
        ),
        ("d", Value::Int(3)),
    ]);

    let result = umt_get_objects_diff(&obj1, &[&obj2]);

    let expected = object_of(vec![(
        "a",
        Value::Object(object_of(vec![("c", Value::Int(2)), ("d", Value::Int(4))])),
    )]);
    assert_eq!(result, expected);
}

#[test]
fn test_should_exclude_key_when_nested_diff_is_empty() {
    let obj = object_of(vec![(
        "a",
        Value::Object(object_of(vec![("b", Value::Int(1))])),
    )]);

    assert!(umt_get_objects_diff(&obj, &[&obj]).is_empty());
}

#[test]
fn test_should_handle_mixed_nested_and_primitive_values() {
    let obj1 = object_of(vec![(
        "a",
        Value::Object(object_of(vec![("b", Value::Int(1))])),
    )]);
    let obj2 = object_of(vec![("a", Value::String("hello".to_string()))]);

    assert_eq!(umt_get_objects_diff(&obj1, &[&obj2]), obj2);
}

#[test]
fn test_should_treat_nan_values_as_equal() {
    let nan = object_of(vec![("a", Value::Float(f64::NAN))]);
    let one = object_of(vec![("a", Value::Int(1))]);

    assert!(umt_get_objects_diff(&nan, &[&nan]).is_empty());

    let result = umt_get_objects_diff(&one, &[&nan]);
    assert!(matches!(result.get("a"), Some(Value::Float(value)) if value.is_nan()));
}

#[test]
fn test_should_treat_int_and_float_with_same_value_as_equal() {
    let int = object_of(vec![("a", Value::Int(1))]);
    let float = object_of(vec![("a", Value::Float(1.0))]);
    let other = object_of(vec![("a", Value::Float(1.5))]);

    assert!(umt_get_objects_diff(&int, &[&float]).is_empty());
    assert!(umt_get_objects_diff(&float, &[&int]).is_empty());
    assert_eq!(umt_get_objects_diff(&int, &[&other]), other);
    assert!(umt_get_objects_diff(&other, &[&other]).is_empty());
}

#[test]
fn test_should_not_mutate_original_objects() {
    let obj1 = object_of(vec![("a", Value::Int(1))]);
    let obj2 = object_of(vec![("a", Value::Int(1)), ("b", Value::Int(2))]);
    let obj1_copy = obj1.clone();
    let obj2_copy = obj2.clone();

    umt_get_objects_diff(&obj1, &[&obj2]);

    assert_eq!(obj1, obj1_copy);
    assert_eq!(obj2, obj2_copy);
}
