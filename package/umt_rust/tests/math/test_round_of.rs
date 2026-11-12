use umt_rust::math::umt_round_of;

#[test]
fn test_round_of() {
    assert_eq!(umt_round_of(3.14159, 2), 3.14);
    assert_eq!(umt_round_of(3.145, 2), 3.15);
    assert_eq!(umt_round_of(0.0, 5), 0.0);
    assert_eq!(umt_round_of(-2.71828, 3), -2.718);
    assert_eq!(umt_round_of(123456.789, 1), 123456.8);
}

#[test]
fn test_round_of_js_ties_and_edges() {
    assert_eq!(umt_round_of(2.5, 0), 3.0);
    assert_eq!(umt_round_of(-2.5, 0), -2.0);
    assert_eq!(umt_round_of(0.49999999999999994, 0), 0.0);
    assert_eq!(umt_round_of(4503599627370497.0, 0), 4503599627370497.0);
    assert!(umt_round_of(f64::NAN, 2).is_nan());
    assert_eq!(umt_round_of(f64::INFINITY, 2), f64::INFINITY);
}
