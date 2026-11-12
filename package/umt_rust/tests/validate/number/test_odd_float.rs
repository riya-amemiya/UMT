//! Tests for odd number validation with non-finite and very large values

use umt_rust::validate::number::umt_odd;

#[test]
fn test_odd_accepts_non_finite_numbers_like_the_remainder_check() {
    let validator = umt_odd(None);
    assert!((validator.validate)(&f64::NAN));
    assert!((validator.validate)(&f64::INFINITY));
    assert!((validator.validate)(&f64::NEG_INFINITY));
}

#[test]
fn test_odd_handles_values_beyond_the_integer_range() {
    let validator = umt_odd(None);
    assert!(!(validator.validate)(&1e300));
    assert!(!(validator.validate)(&-1e300));
    assert!((validator.validate)(&9_007_199_254_740_991.0));
}

#[test]
fn test_odd_handles_signed_zero() {
    let validator = umt_odd(None);
    assert!(!(validator.validate)(&-0.0));
}
