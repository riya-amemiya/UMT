//! Tests for one-of (literal union) string validation

use umt_rust::validate::string::umt_one_of;

#[test]
fn test_one_of_validates_a_value_included_in_the_allowed_list() {
    let validator = umt_one_of(
        &["standard", "squat", "decanter", "round", "tall", "flask"],
        None,
    );
    assert!(validator("standard").validate);
    assert!(validator("flask").validate);
    assert!(!validator("unknown").validate);
}

#[test]
fn test_one_of_returns_the_custom_message_when_validation_fails() {
    let validator = umt_one_of(&["a", "b"], Some("must be a or b".to_string()));
    assert_eq!(validator("c").message, "must be a or b");
    assert!(validator("a").validate);
    assert_eq!(validator("a").message, "");
}

#[test]
fn test_one_of_returns_an_empty_message_when_no_message_is_provided() {
    let validator = umt_one_of(&["x", "y"], None);
    assert_eq!(validator("z").message, "");
    assert!(!validator("z").validate);
}

#[test]
fn test_one_of_rejects_an_empty_allowed_list() {
    let validator = umt_one_of(&[], None);
    assert!(!validator("anything").validate);
}

#[test]
fn test_one_of_treats_the_allowed_list_as_case_sensitive() {
    let validator = umt_one_of(&["Tall"], None);
    assert!(validator("Tall").validate);
    assert!(!validator("tall").validate);
}

#[test]
fn test_one_of_exposes_the_value_through_the_type_value_field() {
    let validator = umt_one_of(&["a", "b"], None);
    assert_eq!(validator("a").type_value, "a");
    assert_eq!(validator("b").type_value, "b");
}

#[test]
fn test_one_of_is_reusable_across_calls() {
    let validator = umt_one_of(&["standard", "squat"], Some("invalid".to_string()));
    let first = validator("standard");
    let second = validator("wine");
    assert!(first.validate);
    assert!(!second.validate);
    assert_eq!(first.message, "");
    assert_eq!(second.message, "invalid");
}
