//! Tests for never function

use umt_rust::validate::umt_never;

#[test]
fn test_never_rejects_every_value() {
    assert!(!umt_never(None)(0).validate);
    assert!(!umt_never(None)("").validate);
    assert!(!umt_never(None)(()).validate);
    assert!(!umt_never(None)(true).validate);
}

#[test]
fn test_never_exposes_the_never_tag() {
    let validator = umt_never(None);
    assert_eq!(validator(42).type_value, "never");
}

#[test]
fn test_never_returns_default_message_without_custom_message() {
    let validator = umt_never::<i32>(None);
    assert_eq!(validator(0).message, "");
}

#[test]
fn test_never_returns_the_configured_message_on_failure() {
    let validator = umt_never(Some("must never be set".to_string()));
    assert_eq!(validator(0).message, "must never be set");
}

#[test]
fn test_never_is_reusable_across_calls() {
    let validator = umt_never(Some("forbidden".to_string()));
    let first = validator("a");
    let second = validator("b");
    assert!(!first.validate);
    assert!(!second.validate);
    assert_eq!(first.message, "forbidden");
    assert_eq!(second.message, "forbidden");
}
