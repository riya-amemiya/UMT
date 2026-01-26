use umt_rust::math::umt_min;

#[test]
fn test_min_basic() {
    assert_eq!(umt_min(&[1.0, 2.0, 3.0]), 1.0);
    assert_eq!(umt_min(&[3.0, 2.0, 1.0]), 1.0);
}

#[test]
fn test_min_negative() {
    assert_eq!(umt_min(&[-1.0, -2.0, -3.0]), -3.0);
}

#[test]
fn test_min_with_duplicates() {
    assert_eq!(umt_min(&[1.0, 1.0, 1.0]), 1.0);
    assert_eq!(umt_min(&[1.0, 2.0, 2.0, 3.0, 3.0]), 1.0);
}

#[test]
fn test_min_single() {
    assert_eq!(umt_min(&[5.0]), 5.0);
    assert_eq!(umt_min(&[-5.0]), -5.0);
}

#[test]
fn test_min_decimals() {
    assert_eq!(umt_min(&[1.5, 2.5, 1.1]), 1.1);
    assert_eq!(umt_min(&[-1.5, -2.5, -1.1]), -2.5);
}

// Note: Rust's NaN comparison behavior differs from JavaScript
// In Rust, NaN comparisons always return false, so NaN values are effectively skipped
#[test]
fn test_min_with_nan_only() {
    // Only NaN returns INFINITY since no valid number is found
    assert_eq!(umt_min(&[f64::NAN]), f64::INFINITY);
}

#[test]
fn test_min_with_nan_mixed() {
    // NaN is skipped, so min of [1.0, NaN, 3.0] is 1.0
    assert_eq!(umt_min(&[1.0, f64::NAN, 3.0]), 1.0);
}

#[test]
fn test_min_with_infinity() {
    assert_eq!(umt_min(&[1.0, f64::INFINITY, 3.0]), 1.0);
    assert_eq!(umt_min(&[f64::INFINITY, f64::INFINITY]), f64::INFINITY);
}

#[test]
fn test_min_with_neg_infinity() {
    assert_eq!(umt_min(&[f64::NEG_INFINITY, 0.0, 1.0]), f64::NEG_INFINITY);
}

#[test]
fn test_min_empty() {
    assert_eq!(umt_min(&[]), f64::INFINITY);
}
