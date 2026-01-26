use umt_rust::validate::number::*;

#[test]
fn test_validate_number() {
    let result = umt_validate_number(42.0, &[], None);
    assert!(result.validate);
}

#[test]
fn test_validate_number_nan() {
    let result = umt_validate_number(f64::NAN, &[], None);
    assert!(!result.validate);
}

#[test]
fn test_even() {
    let validator = umt_even(None);
    assert!((validator.validate)(&4.0));
    assert!(!(validator.validate)(&3.0));
    assert!(!(validator.validate)(&3.5));
}

#[test]
fn test_odd() {
    let validator = umt_odd(None);
    assert!((validator.validate)(&3.0));
    assert!(!(validator.validate)(&4.0));
}

#[test]
fn test_min_value() {
    let validator = umt_min_value(10.0, None);
    assert!((validator.validate)(&10.0));
    assert!((validator.validate)(&15.0));
    assert!(!(validator.validate)(&5.0));
}

#[test]
fn test_max_value() {
    let validator = umt_max_value(10.0, None);
    assert!((validator.validate)(&10.0));
    assert!((validator.validate)(&5.0));
    assert!(!(validator.validate)(&15.0));
}
