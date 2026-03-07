use umt_rust::obj;
use umt_rust::object::{umt_is_plain_object, Value};

#[test]
fn test_should_return_true_for_object_literals() {
    assert!(umt_is_plain_object(&obj! {}));
    assert!(umt_is_plain_object(&obj! {"a" => 1}));
}

#[test]
fn test_should_return_false_for_non_objects() {
    assert!(!umt_is_plain_object(&Value::Null));
    assert!(!umt_is_plain_object(&Value::Int(1)));
    assert!(!umt_is_plain_object(&Value::String("string".to_string())));
    assert!(!umt_is_plain_object(&Value::Bool(true)));
}

#[test]
fn test_should_return_false_for_arrays() {
    assert!(!umt_is_plain_object(&Value::Array(vec![])));
}
