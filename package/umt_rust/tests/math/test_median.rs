use umt_rust::math::umt_median;

#[test]
fn test_median_even() {
    assert_eq!(umt_median(&[1.0, 2.0, 3.0, 4.0]), 2.5);
}

#[test]
fn test_median_odd() {
    assert_eq!(umt_median(&[1.0, 3.0, 3.0, 6.0, 7.0, 8.0, 9.0]), 6.0);
}

#[test]
fn test_median_empty() {
    assert!(umt_median(&[]).is_nan());
}

#[test]
fn test_median_unsorted() {
    assert_eq!(umt_median(&[9.0, 1.0, 5.0, 3.0, 6.0]), 5.0);
}

#[test]
fn test_median_single() {
    assert_eq!(umt_median(&[5.0]), 5.0);
    assert_eq!(umt_median(&[-10.0]), -10.0);
}

#[test]
fn test_median_two_elements() {
    assert_eq!(umt_median(&[1.0, 3.0]), 2.0);
    assert_eq!(umt_median(&[10.0, 20.0]), 15.0);
}

#[test]
fn test_median_with_infinity() {
    assert_eq!(umt_median(&[1.0, 2.0, f64::INFINITY]), 2.0);
    assert_eq!(umt_median(&[f64::NEG_INFINITY, 0.0, 1.0]), 0.0);
}

#[test]
fn test_median_same_values() {
    assert_eq!(umt_median(&[5.0, 5.0, 5.0, 5.0]), 5.0);
}

#[test]
fn test_median_negative() {
    assert_eq!(umt_median(&[-5.0, -3.0, -1.0, 0.0, 2.0]), -1.0);
}
