use umt_rust::math::umt_standard_deviation;

#[test]
fn test_standard_deviation_empty_is_zero() {
    assert_eq!(umt_standard_deviation(&[]), 0.0);
}

#[test]
fn test_standard_deviation_single_value_is_zero() {
    assert_eq!(umt_standard_deviation(&[5.0]), 0.0);
}

#[test]
fn test_standard_deviation_two_values() {
    assert_eq!(umt_standard_deviation(&[1.0, 2.0]), 0.5);
}
