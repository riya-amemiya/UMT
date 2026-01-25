use umt_rust::math::umt_max;

#[test]
fn test_max_basic() {
    assert_eq!(umt_max(&[1.0, 2.0, 3.0]), 3.0);
    assert_eq!(umt_max(&[3.0, 2.0, 1.0]), 3.0);
}

#[test]
fn test_max_negative() {
    assert_eq!(umt_max(&[-1.0, -2.0, -3.0]), -1.0);
}

#[test]
fn test_max_with_duplicates() {
    assert_eq!(umt_max(&[1.0, 1.0, 1.0]), 1.0);
    assert_eq!(umt_max(&[1.0, 2.0, 2.0, 3.0, 3.0]), 3.0);
}

#[test]
fn test_max_single() {
    assert_eq!(umt_max(&[5.0]), 5.0);
    assert_eq!(umt_max(&[-5.0]), -5.0);
}

#[test]
fn test_max_decimals() {
    assert_eq!(umt_max(&[1.5, 2.5, 1.1]), 2.5);
    assert_eq!(umt_max(&[-1.5, -2.5, -1.1]), -1.1);
}

// Note: Rust's NaN comparison behavior differs from JavaScript
// In Rust, NaN comparisons always return false, so NaN values are effectively skipped
#[test]
fn test_max_with_nan_only() {
    // Only NaN returns NEG_INFINITY since no valid number is found
    assert_eq!(umt_max(&[f64::NAN]), f64::NEG_INFINITY);
}

#[test]
fn test_max_with_nan_mixed() {
    // NaN is skipped, so max of [1.0, NaN, 3.0] is 3.0
    assert_eq!(umt_max(&[1.0, f64::NAN, 3.0]), 3.0);
}

#[test]
fn test_max_with_infinity() {
    assert_eq!(umt_max(&[1.0, f64::INFINITY, 3.0]), f64::INFINITY);
    assert_eq!(umt_max(&[f64::NEG_INFINITY, 0.0, 1.0]), 1.0);
}

#[test]
fn test_max_with_neg_infinity() {
    assert_eq!(
        umt_max(&[f64::NEG_INFINITY, f64::NEG_INFINITY]),
        f64::NEG_INFINITY
    );
}

#[test]
fn test_max_empty() {
    assert_eq!(umt_max(&[]), f64::NEG_INFINITY);
}
