use umt_rust::math::umt_addition;

#[test]
fn test_addition_integer_fast_path() {
    assert_eq!(umt_addition(&[1.0, 2.0, 3.0]), 6.0);
    assert_eq!(umt_addition(&[-5.0, 5.0]), 0.0);
}

#[test]
fn test_addition_large_integers_stay_exact() {
    assert_eq!(umt_addition(&[1e21, 1.0]), 1e21 + 1.0);
}

#[test]
fn test_addition_mixed_decimal_lengths() {
    assert_eq!(umt_addition(&[0.1, 0.25, 1.0]), 1.35);
}

#[test]
fn test_addition_empty_is_zero() {
    assert_eq!(umt_addition(&[]), 0.0);
}

#[test]
fn test_addition_non_finite_values() {
    assert_eq!(umt_addition(&[f64::INFINITY, 1.0]), f64::INFINITY);
    assert!(umt_addition(&[f64::NAN, 1.0]).is_nan());
}
