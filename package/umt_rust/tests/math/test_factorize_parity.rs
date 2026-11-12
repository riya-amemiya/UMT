use umt_rust::math::umt_factorize;

#[test]
fn test_factorize_one_is_empty() {
    assert_eq!(umt_factorize(1), Vec::<i32>::new());
    assert_eq!(umt_factorize(-1), Vec::<i32>::new());
    assert_eq!(umt_factorize(0), Vec::<i32>::new());
}

#[test]
fn test_factorize_negative_uses_absolute_value() {
    assert_eq!(umt_factorize(-12), vec![2, 2, 3]);
}

#[test]
fn test_factorize_composite_and_prime() {
    assert_eq!(umt_factorize(12), vec![2, 2, 3]);
    assert_eq!(umt_factorize(97), vec![97]);
}

#[test]
fn test_factorize_largest_prime_does_not_overflow() {
    assert_eq!(umt_factorize(i32::MAX), vec![i32::MAX]);
}
