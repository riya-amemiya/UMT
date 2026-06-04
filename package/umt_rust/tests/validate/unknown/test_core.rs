//! Tests for unknown core validation

use umt_rust::validate::unknown::umt_unknown;

#[test]
fn test_unknown_accepts_any_value() {
    let validator = umt_unknown::<i32>();
    assert!(validator(0).validate);

    let string_validator = umt_unknown::<&str>();
    assert!(string_validator("").validate);

    let option_validator = umt_unknown::<Option<i32>>();
    assert!(option_validator(None).validate);
    assert!(option_validator(Some(123)).validate);
}

#[test]
fn test_unknown_exposes_the_unknown_tag_through_the_value_type_field() {
    let validator = umt_unknown::<i32>();
    assert_eq!(validator(42).value_type, "unknown");
}

#[test]
fn test_unknown_returns_empty_message() {
    let validator = umt_unknown::<i32>();
    assert_eq!(validator(1).message, "");
}
