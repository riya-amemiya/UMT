use umt_rust::math::{umt_gcd, umt_gcd_multiple};

#[test]
fn test_two_positive_integers() {
    assert_eq!(umt_gcd(56, 48), 8);
    assert_eq!(umt_gcd(12, 18), 6);
    assert_eq!(umt_gcd(48, 18), 6);
}

#[test]
fn test_more_than_two_positive_integers() {
    assert_eq!(umt_gcd_multiple(&[56, 48, 32]), 8);
    assert_eq!(umt_gcd_multiple(&[56, 48, 32, 24]), 8);
    assert_eq!(umt_gcd_multiple(&[12, 18, 24]), 6);
}

#[test]
fn test_contains_zero() {
    assert_eq!(umt_gcd(56, 0), 56);
    assert_eq!(umt_gcd(0, 56), 56);
    assert_eq!(umt_gcd(0, 0), 0);
}

#[test]
fn test_contains_one() {
    assert_eq!(umt_gcd(56, 1), 1);
    assert_eq!(umt_gcd(1, 56), 1);
    assert_eq!(umt_gcd(1, 1), 1);
}

#[test]
fn test_contains_negative_numbers() {
    assert_eq!(umt_gcd(-56, 48), 8);
    assert_eq!(umt_gcd(56, -48), 8);
    assert_eq!(umt_gcd(-56, -48), 8);
    assert_eq!(umt_gcd_multiple(&[-56, 48, -32]), 8);
}

#[test]
fn test_handles_large_numbers() {
    assert_eq!(umt_gcd(1_000_000, 500_000), 500_000);
}

#[test]
fn test_handles_edge_cases() {
    assert_eq!(umt_gcd(1, 2), 1);
    assert_eq!(umt_gcd(2, 3), 1);
    assert_eq!(umt_gcd(7, 11), 1);
}

#[test]
fn test_handles_same_numbers() {
    assert_eq!(umt_gcd(42, 42), 42);
    assert_eq!(umt_gcd_multiple(&[100, 100, 100]), 100);
}

#[test]
fn test_handles_small_numbers() {
    assert_eq!(umt_gcd(2, 4), 2);
    assert_eq!(umt_gcd(3, 9), 3);
    assert_eq!(umt_gcd(5, 25), 5);
}
