use umt_rust::math::umt_deviation_value;

#[test]
fn test_deviation_value() {
    assert_eq!(umt_deviation_value(100.0, 50.0, 10.0), 100.0);
    assert_eq!(umt_deviation_value(100.0, 50.0, 20.0), 75.0);
    assert_eq!(umt_deviation_value(0.0, 50.0, 10.0), 0.0);
    assert_eq!(umt_deviation_value(100.0, 0.0, 10.0), 150.0);
    assert_eq!(umt_deviation_value(-100.0, 50.0, 10.0), -100.0);
}
