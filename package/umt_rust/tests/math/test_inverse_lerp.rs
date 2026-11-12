use umt_rust::math::umt_inverse_lerp;

#[test]
fn test_inverse_lerp_endpoints() {
    assert_eq!(umt_inverse_lerp(10.0, 20.0, 10.0), 0.0);
    assert_eq!(umt_inverse_lerp(10.0, 20.0, 20.0), 1.0);
}

#[test]
fn test_inverse_lerp_inside_range() {
    assert_eq!(umt_inverse_lerp(0.0, 10.0, 5.0), 0.5);
    assert_eq!(umt_inverse_lerp(10.0, 20.0, 12.5), 0.25);
    assert_eq!(umt_inverse_lerp(20.0, 10.0, 17.5), 0.25);
}

#[test]
fn test_inverse_lerp_outside_range() {
    assert_eq!(umt_inverse_lerp(0.0, 10.0, 20.0), 2.0);
    assert_eq!(umt_inverse_lerp(0.0, 10.0, -5.0), -0.5);
}

#[test]
fn test_inverse_lerp_empty_range() {
    assert_eq!(umt_inverse_lerp(5.0, 5.0, 7.0), 0.0);
    assert_eq!(umt_inverse_lerp(5.0, 5.0, 5.0), 0.0);
}

#[test]
fn test_inverse_lerp_nan() {
    assert!(umt_inverse_lerp(0.0, 10.0, f64::NAN).is_nan());
}
