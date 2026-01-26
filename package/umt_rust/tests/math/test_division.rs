use umt_rust::math::{umt_division, umt_division_with_remainder};

#[test]
fn test_division_basic() {
    assert_eq!(umt_division(10.0, 2.0), 5.0);
    assert_eq!(umt_division(30.0, 5.0), 6.0);
}

#[test]
fn test_division_large_by_small() {
    assert_eq!(umt_division(1000.0, 10.0), 100.0);
    assert_eq!(umt_division(10000.0, 100.0), 100.0);
}

#[test]
fn test_division_small_by_large() {
    assert_eq!(umt_division(10.0, 100.0), 0.1);
    assert_eq!(umt_division(1.0, 1000.0), 0.001);
}

#[test]
fn test_division_with_remainder_floor() {
    let result = umt_division(7.0, 2.0);
    assert!((result - 3.5).abs() < 1e-10);
}

#[test]
fn test_division_with_remainder_result() {
    let result = umt_division_with_remainder(10.0, 3.0).unwrap();
    assert_eq!(result.quotient, 3);
    assert_eq!(result.remainder, 1);
}

#[test]
fn test_division_with_remainder_7_2() {
    let result = umt_division_with_remainder(7.0, 2.0).unwrap();
    assert_eq!(result.quotient, 3);
    assert_eq!(result.remainder, 1);
}

#[test]
fn test_division_with_remainder_3_5() {
    let result = umt_division_with_remainder(3.0, 5.0).unwrap();
    assert_eq!(result.quotient, 0);
    assert_eq!(result.remainder, 3);
}

#[test]
fn test_division_with_remainder_large() {
    let result = umt_division_with_remainder(1000.0, 3.0).unwrap();
    assert_eq!(result.quotient, 333);
    assert_eq!(result.remainder, 1);
}

#[test]
fn test_division_decimals() {
    let result = umt_division(10.5, 2.1);
    assert!((result - 5.0).abs() < 1e-10);
}

#[test]
fn test_division_decimals_0_1_0_2() {
    let result = umt_division(0.1, 0.2);
    assert!((result - 0.5).abs() < 1e-10);
}

#[test]
fn test_division_decimals_0_001_0_1() {
    let result = umt_division(0.001, 0.1);
    assert!((result - 0.01).abs() < 1e-10);
}

#[test]
fn test_division_decimals_0_1_0_001() {
    let result = umt_division(0.1, 0.001);
    assert!((result - 100.0).abs() < 1e-10);
}

#[test]
fn test_division_same_decimal_length() {
    let result = umt_division(1.1, 2.2);
    assert!((result - 0.5).abs() < 1e-10);
}

#[test]
fn test_division_by_zero() {
    assert!(umt_division(1.0, 0.0).is_nan());
    assert!(umt_division(-1.0, 0.0).is_nan());
    assert!(umt_division(0.0, 0.0).is_nan());
}

#[test]
fn test_division_with_remainder_by_zero() {
    assert!(umt_division_with_remainder(10.0, 0.0).is_none());
    assert!(umt_division_with_remainder(0.0, 0.0).is_none());
}

#[test]
fn test_division_zero_dividend() {
    assert_eq!(umt_division(0.0, 1.0), 0.0);
}

#[test]
fn test_division_negative() {
    assert_eq!(umt_division(-10.0, 2.0), -5.0);
    assert_eq!(umt_division(10.0, -2.0), -5.0);
    assert_eq!(umt_division(-10.0, -2.0), 5.0);
}

#[test]
fn test_division_negative_decimals() {
    let result = umt_division(-0.16, 0.2);
    assert!((result - (-0.8)).abs() < 1e-10);
}

#[test]
fn test_division_decimals_negative_divisor() {
    let result = umt_division(0.16, -0.2);
    assert!((result - (-0.8)).abs() < 1e-10);
}

#[test]
fn test_division_both_negative_decimals() {
    let result = umt_division(-0.16, -0.2);
    assert!((result - 0.8).abs() < 1e-10);
}
