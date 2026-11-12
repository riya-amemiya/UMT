use umt_rust::math::umt_max;

#[test]
fn test_max_nan_anywhere_is_nan() {
    assert!(umt_max(&[f64::NAN]).is_nan());
    assert!(umt_max(&[1.0, f64::NAN, 3.0]).is_nan());
    assert!(umt_max(&[f64::NAN, 1.0]).is_nan());
    assert!(umt_max(&[1.0, f64::NAN]).is_nan());
}

#[test]
fn test_max_without_nan() {
    assert_eq!(umt_max(&[1.0, 3.0, 2.0]), 3.0);
    assert_eq!(umt_max(&[]), f64::NEG_INFINITY);
}
