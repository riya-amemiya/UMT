use umt_rust::math::umt_get_decimal_length;

#[test]
fn test_get_decimal_length_integral_values() {
    assert_eq!(umt_get_decimal_length(1.0), 0);
    assert_eq!(umt_get_decimal_length(-100.0), 0);
    assert_eq!(umt_get_decimal_length(0.0), 0);
}

#[test]
fn test_get_decimal_length_fractional_values() {
    assert_eq!(umt_get_decimal_length(1.23), 2);
    assert_eq!(umt_get_decimal_length(-0.125), 3);
}

#[test]
fn test_get_decimal_length_non_finite_values() {
    assert_eq!(umt_get_decimal_length(f64::NAN), 0);
    assert_eq!(umt_get_decimal_length(f64::INFINITY), 0);
}
