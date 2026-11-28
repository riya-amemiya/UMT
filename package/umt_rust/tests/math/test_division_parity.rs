use umt_rust::math::umt_division;

#[test]
fn test_division_integer_fast_path() {
    assert_eq!(umt_division(10.0, 2.0), 5.0);
    assert_eq!(umt_division(-10.0, 2.0), -5.0);
    assert_eq!(umt_division(10.0, -2.0), -5.0);
    assert_eq!(umt_division(-10.0, -2.0), 5.0);
}

#[test]
fn test_division_large_integers_stay_exact() {
    assert_eq!(umt_division(1e9, 1e9), 1.0);
}

#[test]
fn test_division_mixed_decimal_lengths() {
    assert_eq!(umt_division(2.0, 0.5), 4.0);
    assert_eq!(umt_division(0.1, 0.2), 0.5);
}

#[test]
fn test_division_zero_divisor_is_nan() {
    assert!(umt_division(1.0, 0.0).is_nan());
    assert!(umt_division(-1.0, 0.0).is_nan());
    assert!(umt_division(0.0, 0.0).is_nan());
}

#[test]
fn test_division_non_finite_values() {
    assert_eq!(umt_division(f64::INFINITY, 2.0), f64::INFINITY);
    assert!(umt_division(f64::NAN, 2.0).is_nan());
}
