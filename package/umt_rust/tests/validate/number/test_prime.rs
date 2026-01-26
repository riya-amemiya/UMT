//! Tests for prime number validation

use umt_rust::validate::number::umt_prime;

#[test]
fn test_prime_returns_true_for_prime_numbers() {
    let validator = umt_prime(None);
    assert!((validator.validate)(&2.0));
    assert!((validator.validate)(&3.0));
    assert!((validator.validate)(&5.0));
    assert!((validator.validate)(&7.0));
}

#[test]
fn test_prime_returns_false_for_non_prime_numbers() {
    let validator = umt_prime(None);
    assert!(!(validator.validate)(&4.0));
    assert!(!(validator.validate)(&6.0));
    assert!(!(validator.validate)(&8.0));
    assert!(!(validator.validate)(&9.0));
}

#[test]
fn test_prime_returns_custom_message() {
    let custom_message = "This is not a prime number";
    let validator = umt_prime(Some(custom_message.to_string()));
    assert!(!(validator.validate)(&4.0));
    assert_eq!(validator.message, Some(custom_message.to_string()));
}

#[test]
fn test_prime_with_zero_and_one() {
    let validator = umt_prime(None);
    assert!(!(validator.validate)(&0.0));
    assert!(!(validator.validate)(&1.0));
}

#[test]
fn test_prime_with_larger_primes() {
    let validator = umt_prime(None);
    assert!((validator.validate)(&11.0));
    assert!((validator.validate)(&13.0));
    assert!((validator.validate)(&17.0));
    assert!((validator.validate)(&19.0));
    assert!((validator.validate)(&23.0));
    assert!((validator.validate)(&97.0));
}

#[test]
fn test_prime_with_larger_composites() {
    let validator = umt_prime(None);
    assert!(!(validator.validate)(&12.0));
    assert!(!(validator.validate)(&14.0));
    assert!(!(validator.validate)(&15.0));
    assert!(!(validator.validate)(&16.0));
    assert!(!(validator.validate)(&100.0));
}

use umt_rust::validate::number::*;

#[test]
fn test_custom_message() {
    let custom_message = "This is not a prime number";
    let validator = umt_prime(Some(custom_message.to_string()));
    assert!(!(validator.validate)(&4.0));
    assert_eq!(validator.message, Some(custom_message.to_string()));
}

#[test]
fn test_returns_false_for_non_prime_numbers() {
    let validator = umt_prime(None);
    assert!(!(validator.validate)(&4.0));
    assert!(!(validator.validate)(&6.0));
    assert!(!(validator.validate)(&8.0));
    assert!(!(validator.validate)(&9.0));
}

#[test]
fn test_returns_false_for_one_and_less() {
    let validator = umt_prime(None);
    assert!(!(validator.validate)(&1.0));
    assert!(!(validator.validate)(&0.0));
    assert!(!(validator.validate)(&-2.0));
}

#[test]
fn test_returns_true_for_prime_numbers() {
    let validator = umt_prime(None);
    assert!((validator.validate)(&2.0));
    assert!((validator.validate)(&3.0));
    assert!((validator.validate)(&5.0));
    assert!((validator.validate)(&7.0));
}
