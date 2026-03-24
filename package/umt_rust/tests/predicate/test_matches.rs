use std::collections::HashMap;

use serde_json::Value;
use umt_rust::predicate::umt_matches;

#[test]
fn returns_true_when_object_matches_pattern() {
    let mut pattern = HashMap::new();
    pattern.insert("role".to_string(), Value::String("admin".to_string()));
    let is_admin = umt_matches(pattern);

    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Value::String("Alice".to_string()));
    obj.insert("role".to_string(), Value::String("admin".to_string()));
    assert!(is_admin(&obj));
}

#[test]
fn returns_false_when_object_does_not_match() {
    let mut pattern = HashMap::new();
    pattern.insert("role".to_string(), Value::String("admin".to_string()));
    let is_admin = umt_matches(pattern);

    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Value::String("Bob".to_string()));
    obj.insert("role".to_string(), Value::String("user".to_string()));
    assert!(!is_admin(&obj));
}

#[test]
fn matches_multiple_properties() {
    let mut pattern = HashMap::new();
    pattern.insert("a".to_string(), Value::from(1));
    pattern.insert("b".to_string(), Value::from(2));
    let matcher = umt_matches(pattern);

    let mut obj1 = HashMap::new();
    obj1.insert("a".to_string(), Value::from(1));
    obj1.insert("b".to_string(), Value::from(2));
    obj1.insert("c".to_string(), Value::from(3));
    assert!(matcher(&obj1));

    let mut obj2 = HashMap::new();
    obj2.insert("a".to_string(), Value::from(1));
    obj2.insert("b".to_string(), Value::from(3));
    assert!(!matcher(&obj2));

    let mut obj3 = HashMap::new();
    obj3.insert("a".to_string(), Value::from(2));
    obj3.insert("b".to_string(), Value::from(2));
    assert!(!matcher(&obj3));
}

#[test]
fn uses_strict_equality() {
    let mut pattern = HashMap::new();
    pattern.insert("value".to_string(), Value::from(0));
    let matcher = umt_matches(pattern);

    let mut obj_match = HashMap::new();
    obj_match.insert("value".to_string(), Value::from(0));
    assert!(matcher(&obj_match));

    let mut obj_false = HashMap::new();
    obj_false.insert("value".to_string(), Value::Bool(false));
    assert!(!matcher(&obj_false));

    let mut obj_str = HashMap::new();
    obj_str.insert("value".to_string(), Value::String("".to_string()));
    assert!(!matcher(&obj_str));

    let mut obj_null = HashMap::new();
    obj_null.insert("value".to_string(), Value::Null);
    assert!(!matcher(&obj_null));
}

#[test]
fn returns_true_for_empty_pattern() {
    let always_match = umt_matches(HashMap::new());

    let mut obj = HashMap::new();
    obj.insert("anything".to_string(), Value::String("goes".to_string()));
    assert!(always_match(&obj));
    assert!(always_match(&HashMap::new()));
}

#[test]
fn handles_missing_keys_in_target_object() {
    let mut pattern = HashMap::new();
    pattern.insert("x".to_string(), Value::from(1));
    let matcher = umt_matches(pattern);

    assert!(!matcher(&HashMap::new()));

    let mut obj = HashMap::new();
    obj.insert("y".to_string(), Value::from(1));
    assert!(!matcher(&obj));
}

#[test]
fn handles_null_pattern_values() {
    let mut pattern = HashMap::new();
    pattern.insert("a".to_string(), Value::Null);
    let match_null = umt_matches(pattern);

    let mut obj_null = HashMap::new();
    obj_null.insert("a".to_string(), Value::Null);
    assert!(match_null(&obj_null));

    let mut obj_num = HashMap::new();
    obj_num.insert("a".to_string(), Value::from(0));
    assert!(!match_null(&obj_num));
}
