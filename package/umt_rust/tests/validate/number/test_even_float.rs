//! Tests for even number validation with non-finite and very large values

use umt_rust::validate::number::umt_even;

#[test]
fn test_even_rejects_non_finite_numbers() {
    let validator = umt_even(None);
    assert!(!(validator.validate)(&f64::NAN));
    assert!(!(validator.validate)(&f64::INFINITY));
    assert!(!(validator.validate)(&f64::NEG_INFINITY));
}

#[test]
fn test_even_handles_values_beyond_the_integer_range() {
    let validator = umt_even(None);
    assert!((validator.validate)(&1e300));
    assert!((validator.validate)(&-1e300));
    assert!((validator.validate)(&18_446_744_073_709_551_616.0));
}

#[test]
fn test_even_handles_signed_zero() {
    let validator = umt_even(None);
    assert!((validator.validate)(&-0.0));
}
