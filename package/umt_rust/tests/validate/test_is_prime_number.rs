//! Tests for is_prime_number function

use umt_rust::validate::{umt_is_prime_number, umt_is_prime_number_usize};

#[test]
fn test_is_prime_number_returns_false_for_numbers_less_than_or_equal_to_one() {
    assert!(!umt_is_prime_number(0));
    assert!(!umt_is_prime_number(1));
}

#[test]
fn test_is_prime_number_returns_true_for_prime_numbers() {
    assert!(umt_is_prime_number(2));
    assert!(umt_is_prime_number(3));
    assert!(umt_is_prime_number(5));
    assert!(umt_is_prime_number(7));
    assert!(umt_is_prime_number(11));
    assert!(umt_is_prime_number(13));
    assert!(umt_is_prime_number(17));
    assert!(umt_is_prime_number(19));
    assert!(umt_is_prime_number(23));
    assert!(umt_is_prime_number(29));
}

#[test]
fn test_is_prime_number_returns_false_for_composite_numbers() {
    assert!(!umt_is_prime_number(4));
    assert!(!umt_is_prime_number(6));
    assert!(!umt_is_prime_number(8));
    assert!(!umt_is_prime_number(9));
    assert!(!umt_is_prime_number(10));
    assert!(!umt_is_prime_number(12));
    assert!(!umt_is_prime_number(14));
    assert!(!umt_is_prime_number(15));
    assert!(!umt_is_prime_number(16));
    assert!(!umt_is_prime_number(18));
}

#[test]
fn test_is_prime_number_returns_false_for_negative_numbers() {
    assert!(!umt_is_prime_number(-2));
    assert!(!umt_is_prime_number(-7));
    assert!(!umt_is_prime_number(-11));
}

#[test]
fn test_is_prime_number_handles_large_non_prime_numbers() {
    let large_non_prime = 10_000_000_000_000_i64; // 10^13
    assert!(!umt_is_prime_number(large_non_prime));
}

#[test]
fn test_is_prime_number_handles_large_prime_numbers() {
    let large_prime = 982_451_653_i64; // Known prime number
    assert!(umt_is_prime_number(large_prime));
}

#[test]
fn test_is_prime_number_usize() {
    assert!(umt_is_prime_number_usize(2));
    assert!(umt_is_prime_number_usize(17));
    assert!(!umt_is_prime_number_usize(4));
    assert!(!umt_is_prime_number_usize(1));
    assert!(!umt_is_prime_number_usize(0));
}

#[test]
fn test_is_prime_number_more_primes() {
    // Test some more prime numbers
    assert!(umt_is_prime_number(97));
    assert!(umt_is_prime_number(101));
    assert!(umt_is_prime_number(103));
    assert!(umt_is_prime_number(107));
    assert!(umt_is_prime_number(109));
}

#[test]
fn test_is_prime_number_more_composites() {
    // Test some more composite numbers
    assert!(!umt_is_prime_number(100));
    assert!(!umt_is_prime_number(99));
    assert!(!umt_is_prime_number(50));
    assert!(!umt_is_prime_number(25));
}

use umt_rust::validate::*;

#[test]
fn test_is_prime_number() {
    assert!(umt_is_prime_number(2));
    assert!(umt_is_prime_number(3));
    assert!(umt_is_prime_number(5));
    assert!(umt_is_prime_number(7));
    assert!(umt_is_prime_number(11));
    assert!(umt_is_prime_number(13));
    assert!(umt_is_prime_number(17));
    assert!(umt_is_prime_number(97));

    assert!(!umt_is_prime_number(0));
    assert!(!umt_is_prime_number(1));
    assert!(!umt_is_prime_number(4));
    assert!(!umt_is_prime_number(6));
    assert!(!umt_is_prime_number(8));
    assert!(!umt_is_prime_number(9));
    assert!(!umt_is_prime_number(100));
    assert!(!umt_is_prime_number(-3));
}
