use umt_rust::math::umt_to_kelvin;

#[test]
fn test_to_kelvin_is_exact_for_decimal_inputs() {
    assert_eq!(umt_to_kelvin(26.85), 300.0);
    assert_eq!(umt_to_kelvin(0.0), 273.15);
    assert_eq!(umt_to_kelvin(-273.15), 0.0);
    assert_eq!(umt_to_kelvin(100.123), 373.273);
}
