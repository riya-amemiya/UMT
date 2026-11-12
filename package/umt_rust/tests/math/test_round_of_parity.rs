use umt_rust::math::umt_round_of;

#[test]
fn test_round_of_positive_ties_round_up() {
    assert_eq!(umt_round_of(2.5, 0), 3.0);
    assert_eq!(umt_round_of(0.5, 0), 1.0);
    assert_eq!(umt_round_of(0.125, 2), 0.13);
}

#[test]
fn test_round_of_negative_ties_round_toward_positive_infinity() {
    assert_eq!(umt_round_of(-2.5, 0), -2.0);
    assert_eq!(umt_round_of(-1.5, 0), -1.0);
    assert_eq!(umt_round_of(-0.5, 0), 0.0);
}

#[test]
fn test_round_of_just_below_half_rounds_down() {
    assert_eq!(umt_round_of(0.49999999999999994, 0), 0.0);
}

#[test]
fn test_round_of_negative_precision() {
    assert_eq!(umt_round_of(1234.5, -2), 1200.0);
}

#[test]
fn test_round_of_non_finite_values() {
    assert!(umt_round_of(f64::NAN, 2).is_nan());
    assert_eq!(umt_round_of(f64::INFINITY, 2), f64::INFINITY);
    assert_eq!(umt_round_of(f64::NEG_INFINITY, 2), f64::NEG_INFINITY);
}
