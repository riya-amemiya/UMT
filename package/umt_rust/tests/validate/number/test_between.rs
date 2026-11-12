//! Tests for inclusive number range validation

use umt_rust::validate::number::{umt_between, umt_number_validator};

#[test]
fn test_between_accepts_values_inside_the_range_including_bounds() {
    let validator = umt_between(1.0, 10.0, None);
    for value in [1.0, 5.5, 10.0] {
        assert!((validator.validate)(&value), "{value}");
    }
}

#[test]
fn test_between_rejects_values_below_the_minimum() {
    let validator = umt_between(1.0, 10.0, None);
    assert!(!(validator.validate)(&0.999));
    assert!(!(validator.validate)(&-1.0));
}

#[test]
fn test_between_rejects_values_above_the_maximum() {
    let validator = umt_between(1.0, 10.0, None);
    assert!(!(validator.validate)(&10.001));
    assert!(!(validator.validate)(&100.0));
}

#[test]
fn test_between_rejects_nan() {
    assert!(!(umt_between(1.0, 10.0, None).validate)(&f64::NAN));
}

#[test]
fn test_between_accepts_only_the_bound_when_min_equals_max() {
    let validator = umt_between(3.0, 3.0, None);
    assert!((validator.validate)(&3.0));
    assert!(!(validator.validate)(&2.0));
    assert!(!(validator.validate)(&4.0));
}

#[test]
fn test_between_rejects_every_value_when_min_is_greater_than_max() {
    let validator = umt_between(10.0, 1.0, None);
    for value in [1.0, 5.0, 10.0] {
        assert!(!(validator.validate)(&value), "{value}");
    }
}

#[test]
fn test_between_reports_one_message_for_both_bounds() {
    let validator = umt_number_validator(
        vec![umt_between(
            1.0,
            10.0,
            Some("must be between 1 and 10".to_string()),
        )],
        None,
    );
    assert!(validator(5.0).validate);
    assert_eq!(validator(0.0).message, "must be between 1 and 10");
    assert_eq!(validator(11.0).message, "must be between 1 and 10");
    assert_eq!(umt_between(1.0, 10.0, None).message, None);
}
