use umt_rust::math::umt_multiplication;

#[test]
fn test_multiplication_integer_fast_path() {
    assert_eq!(umt_multiplication(&[1.0, 2.0, 3.0]), 6.0);
    assert_eq!(umt_multiplication(&[-5.0, 5.0]), -25.0);
}

#[test]
fn test_multiplication_large_integers_stay_exact() {
    assert_eq!(umt_multiplication(&[1e9, 1e9]), 1e18);
}

#[test]
fn test_multiplication_mixed_decimal_lengths() {
    assert_eq!(umt_multiplication(&[0.1, 0.25, 4.0]), 0.1);
}

#[test]
fn test_multiplication_empty_is_one() {
    assert_eq!(umt_multiplication(&[]), 1.0);
}

#[test]
fn test_multiplication_non_finite_values() {
    assert_eq!(umt_multiplication(&[f64::INFINITY, 2.0]), f64::INFINITY);
    assert!(umt_multiplication(&[f64::NAN, 2.0]).is_nan());
}
