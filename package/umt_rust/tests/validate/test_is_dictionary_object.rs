//! Tests for is_dictionary_object function

use std::collections::HashMap;
use umt_rust::validate::umt_is_dictionary_object;

#[test]
fn test_is_dictionary_object_returns_true_for_empty_object() {
    let map: HashMap<&str, &str> = HashMap::new();
    assert!(umt_is_dictionary_object(&map));
}

#[test]
fn test_is_dictionary_object_returns_true_for_object_with_string_properties() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("foo", "bar");
    map.insert("baz", "qux");
    assert!(umt_is_dictionary_object(&map));
}

#[test]
fn test_is_dictionary_object_returns_true_for_object_with_number_properties() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("foo", 1);
    map.insert("bar", 2);
    assert!(umt_is_dictionary_object(&map));
}

#[test]
fn test_is_dictionary_object_returns_true_for_object_with_boolean_properties() {
    let mut map: HashMap<&str, bool> = HashMap::new();
    map.insert("foo", true);
    map.insert("bar", false);
    assert!(umt_is_dictionary_object(&map));
}

#[test]
fn test_is_dictionary_object_with_string_keys() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());
    assert!(umt_is_dictionary_object(&map));
}

#[test]
fn test_is_dictionary_object_with_integer_keys() {
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    assert!(umt_is_dictionary_object(&map));
}

use umt_rust::validate::*;

#[test]
fn test_is_dictionary_object() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert!(umt_is_dictionary_object(&map));

    let empty: HashMap<String, String> = HashMap::new();
    assert!(umt_is_dictionary_object(&empty));
}
