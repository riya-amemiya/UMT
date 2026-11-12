use umt_rust::math::umt_to_celsius;

#[test]
fn test_to_celsius_is_exact_for_decimal_inputs() {
    assert_eq!(umt_to_celsius(300.0), 26.85);
    assert_eq!(umt_to_celsius(273.15), 0.0);
    assert_eq!(umt_to_celsius(0.0), -273.15);
    assert_eq!(umt_to_celsius(-10.5), -283.65);
}
