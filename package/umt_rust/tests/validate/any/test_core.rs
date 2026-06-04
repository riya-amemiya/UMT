//! Tests for any core validation

use umt_rust::validate::any::umt_any;

#[test]
fn test_any_accepts_any_value() {
    let validator = umt_any::<i32>();
    assert!(validator(0).validate);

    let string_validator = umt_any::<&str>();
    assert!(string_validator("").validate);

    let option_validator = umt_any::<Option<i32>>();
    assert!(option_validator(None).validate);
    assert!(option_validator(Some(123)).validate);
}

#[test]
fn test_any_exposes_the_any_tag_through_the_value_type_field() {
    let validator = umt_any::<i32>();
    assert_eq!(validator(42).value_type, "any");
}

#[test]
fn test_any_returns_empty_message() {
    let validator = umt_any::<i32>();
    assert_eq!(validator(1).message, "");
}
