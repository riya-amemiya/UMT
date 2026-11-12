//! Tests for inclusive string length range validation

use umt_rust::validate::string::{umt_length_between, umt_string_validator};

#[test]
fn test_length_between_accepts_lengths_inside_the_range_including_bounds() {
    let validator = umt_string_validator(vec![umt_length_between(2, 4, None)], None);
    for value in ["ab", "abc", "abcd"] {
        assert!(validator(value).validate, "{value}");
    }
}

#[test]
fn test_length_between_rejects_lengths_outside_the_range_with_one_message() {
    let validator = umt_string_validator(
        vec![umt_length_between(
            2,
            4,
            Some("length must be 2 to 4".to_string()),
        )],
        None,
    );
    for value in ["a", "abcde"] {
        let result = validator(value);
        assert!(!result.validate);
        assert_eq!(result.message, "length must be 2 to 4");
    }
}

#[test]
fn test_length_between_defaults_the_message_to_empty_string() {
    let validator = umt_string_validator(vec![umt_length_between(2, 4, None)], None);
    let result = validator("");
    assert!(!result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_length_between_measures_utf16_code_units() {
    let validator = umt_length_between(2, 2, None);
    assert!((validator.validate)(&"\u{1F600}".to_string()));
    assert!(!(validator.validate)(&"\u{3042}".to_string()));
}

#[test]
fn test_length_between_rejects_every_value_when_min_is_greater_than_max() {
    let validator = umt_length_between(4, 2, None);
    assert!(!(validator.validate)(&"abc".to_string()));
    assert_eq!(validator.type_name, "string");
}
