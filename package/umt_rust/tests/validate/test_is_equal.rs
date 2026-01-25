//! Tests for is_equal function

use umt_rust::validate::{umt_is_equal, umt_is_equal_f64};

#[test]
fn test_is_equal_primitive_values() {
    assert!(umt_is_equal(&1, &1));
    assert!(umt_is_equal(&"test", &"test"));
    assert!(umt_is_equal(&true, &true));
}

#[test]
fn test_is_equal_returns_false_for_different_primitive_values() {
    assert!(!umt_is_equal(&1, &2));
    assert!(!umt_is_equal(&"test", &"other"));
    assert!(!umt_is_equal(&true, &false));
}

#[test]
fn test_is_equal_f64_primitive_values() {
    assert!(umt_is_equal_f64(1.0, 1.0));
}

#[test]
fn test_is_equal_f64_nan_values() {
    // NaN should equal NaN (Object.is semantics)
    assert!(umt_is_equal_f64(f64::NAN, f64::NAN));
}

#[test]
fn test_is_equal_f64_negative_zero() {
    // -0 should not equal +0 (Object.is semantics)
    assert!(!umt_is_equal_f64(-0.0_f64, 0.0_f64));
}

#[test]
fn test_is_equal_references() {
    let obj = vec![1, 2, 3];
    let obj_ref = &obj;
    assert!(umt_is_equal(&obj_ref, &obj_ref));
}

#[test]
fn test_is_equal_strings() {
    let s1 = "hello".to_string();
    let s2 = "hello".to_string();
    assert!(umt_is_equal(&s1, &s2));

    let s3 = "world".to_string();
    assert!(!umt_is_equal(&s1, &s3));
}
