use umt_rust::math::umt_lerp;

#[test]
fn test_lerp_endpoints() {
    assert_eq!(umt_lerp(10.0, 20.0, 0.0), 10.0);
    assert_eq!(umt_lerp(10.0, 20.0, 1.0), 20.0);
}

#[test]
fn test_lerp_interpolation() {
    assert_eq!(umt_lerp(0.0, 10.0, 0.5), 5.0);
    assert_eq!(umt_lerp(10.0, 20.0, 0.25), 12.5);
    assert_eq!(umt_lerp(-10.0, 10.0, 0.75), 5.0);
    assert_eq!(umt_lerp(20.0, 10.0, 0.25), 17.5);
}

#[test]
fn test_lerp_extrapolation() {
    assert_eq!(umt_lerp(0.0, 10.0, 2.0), 20.0);
    assert_eq!(umt_lerp(0.0, 10.0, -0.5), -5.0);
}

#[test]
fn test_lerp_empty_range_and_nan() {
    assert_eq!(umt_lerp(5.0, 5.0, 0.3), 5.0);
    assert!(umt_lerp(0.0, 10.0, f64::NAN).is_nan());
}
