use umt_rust::math::umt_subtract;

#[test]
fn test_subtract_integer_fast_path() {
    assert_eq!(umt_subtract(&[5.0, 2.0, 1.0]), 2.0);
    assert_eq!(umt_subtract(&[-5.0, 5.0]), -10.0);
}

#[test]
fn test_subtract_large_integers_stay_exact() {
    assert_eq!(umt_subtract(&[1e9, 1e9]), 0.0);
}

#[test]
fn test_subtract_mixed_decimal_lengths() {
    assert_eq!(umt_subtract(&[1.0, 0.25, 0.1]), 0.65);
}

#[test]
fn test_subtract_empty_is_zero() {
    assert_eq!(umt_subtract(&[]), 0.0);
}

#[test]
fn test_subtract_non_finite_values() {
    assert_eq!(umt_subtract(&[f64::INFINITY, 2.0]), f64::INFINITY);
    assert!(umt_subtract(&[f64::NAN, 2.0]).is_nan());
}
