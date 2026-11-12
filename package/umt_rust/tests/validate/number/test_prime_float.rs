//! Tests for prime number validation with non-integer and non-finite values

use umt_rust::validate::number::umt_prime;

#[test]
fn test_prime_rejects_non_integer_numbers() {
    let validator = umt_prime(None);
    assert!(!(validator.validate)(&2.5));
    assert!(!(validator.validate)(&3.75));
    assert!(!(validator.validate)(&7.000001));
}

#[test]
fn test_prime_accepts_integer_valued_floats() {
    let validator = umt_prime(None);
    assert!((validator.validate)(&7.0));
    assert!((validator.validate)(&982_451_653.0));
}

#[test]
fn test_prime_rejects_non_finite_numbers() {
    let validator = umt_prime(None);
    assert!(!(validator.validate)(&f64::NAN));
    assert!(!(validator.validate)(&f64::INFINITY));
    assert!(!(validator.validate)(&f64::NEG_INFINITY));
}

#[test]
fn test_prime_rejects_negative_numbers() {
    let validator = umt_prime(None);
    assert!(!(validator.validate)(&-7.0));
    assert!(!(validator.validate)(&-2.5));
}

#[test]
fn test_prime_rejects_values_beyond_the_integer_range() {
    let validator = umt_prime(None);
    assert!(!(validator.validate)(&1e300));
}
