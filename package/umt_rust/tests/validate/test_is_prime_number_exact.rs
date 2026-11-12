//! Tests for the exact square root bound of the prime number checks

use umt_rust::validate::{umt_is_prime_number, umt_is_prime_number_usize};

fn is_prime_by_trial_division(n: i64) -> bool {
    n >= 2 && (2..n).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

#[test]
fn test_is_prime_number_matches_trial_division() {
    for n in -10..5000 {
        assert_eq!(umt_is_prime_number(n), is_prime_by_trial_division(n));
    }
}

#[test]
fn test_is_prime_number_usize_matches_trial_division() {
    for n in 0..5000_usize {
        assert_eq!(
            umt_is_prime_number_usize(n),
            is_prime_by_trial_division(n as i64)
        );
    }
}

#[test]
fn test_is_prime_number_rejects_squares_of_primes() {
    assert!(!umt_is_prime_number(25));
    assert!(!umt_is_prime_number(49));
    assert!(!umt_is_prime_number(121));
    assert!(!umt_is_prime_number(1_000_003 * 1_000_003));
    assert!(!umt_is_prime_number(94_906_249 * 94_906_249));
}

#[test]
fn test_is_prime_number_usize_rejects_squares_of_primes() {
    assert!(!umt_is_prime_number_usize(25));
    assert!(!umt_is_prime_number_usize(49));
    assert!(!umt_is_prime_number_usize(1_000_003 * 1_000_003));
}

#[test]
fn test_is_prime_number_rejects_products_of_twin_candidates() {
    assert!(!umt_is_prime_number(5 * 7));
    assert!(!umt_is_prime_number(11 * 13));
    assert!(!umt_is_prime_number(7 * 11));
    assert!(!umt_is_prime_number_usize(5 * 7));
    assert!(!umt_is_prime_number_usize(7 * 11));
}

#[test]
fn test_is_prime_number_accepts_large_primes() {
    assert!(umt_is_prime_number(2_147_483_647));
    assert!(umt_is_prime_number(4_294_967_291));
    assert!(umt_is_prime_number_usize(2_147_483_647));
    assert!(umt_is_prime_number_usize(3));
    assert!(!umt_is_prime_number_usize(9));
}

#[test]
fn test_is_prime_number_rejects_extreme_values() {
    assert!(!umt_is_prime_number(i64::MIN));
    assert!(!umt_is_prime_number(i64::MAX));
}
