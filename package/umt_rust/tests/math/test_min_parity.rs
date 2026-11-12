use umt_rust::math::umt_min;

#[test]
fn test_min_nan_anywhere_is_nan() {
    assert!(umt_min(&[f64::NAN]).is_nan());
    assert!(umt_min(&[1.0, f64::NAN, 3.0]).is_nan());
    assert!(umt_min(&[f64::NAN, 1.0]).is_nan());
    assert!(umt_min(&[1.0, f64::NAN]).is_nan());
}

#[test]
fn test_min_without_nan() {
    assert_eq!(umt_min(&[3.0, 1.0, 2.0]), 1.0);
    assert_eq!(umt_min(&[]), f64::INFINITY);
}
