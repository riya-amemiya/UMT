use umt_rust::math::umt_is_prime;

#[test]
fn test_is_prime_below_two() {
    assert!(!umt_is_prime(1.0));
    assert!(!umt_is_prime(0.0));
    assert!(!umt_is_prime(-7.0));
}

#[test]
fn test_is_prime_two_and_three() {
    assert!(umt_is_prime(2.0));
    assert!(umt_is_prime(3.0));
}

#[test]
fn test_is_prime_multiples_of_two_or_three() {
    assert!(!umt_is_prime(4.0));
    assert!(!umt_is_prime(9.0));
    assert!(!umt_is_prime(100.0));
}

#[test]
fn test_is_prime_primes() {
    for n in [5.0, 7.0, 29.0, 97.0, 7919.0, 2_147_483_647.0] {
        assert!(umt_is_prime(n), "{n}");
    }
}

#[test]
fn test_is_prime_composites_on_the_wheel() {
    for n in [25.0, 121.0, 49.0, 91.0, 169.0] {
        assert!(!umt_is_prime(n), "{n}");
    }
}

#[test]
fn test_is_prime_non_integers() {
    assert!(!umt_is_prime(7.5));
    assert!(!umt_is_prime(2.000001));
}

#[test]
fn test_is_prime_nan_and_infinity() {
    assert!(!umt_is_prime(f64::NAN));
    assert!(!umt_is_prime(f64::INFINITY));
    assert!(!umt_is_prime(f64::NEG_INFINITY));
}

#[test]
fn test_is_prime_above_two_pow_53() {
    assert!(!umt_is_prime(9_007_199_254_740_994.0));
}
