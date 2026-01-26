use umt_rust::math::umt_subtract;

#[test]
fn test_subtract_equal() {
    assert_eq!(umt_subtract(&[1.0, 1.0]), 0.0);
}

#[test]
fn test_subtract_decimal_from_integer() {
    let result = umt_subtract(&[1.1, 1.0]);
    assert!((result - 0.1).abs() < 1e-10);
}

#[test]
fn test_subtract_integer_from_decimal() {
    let result = umt_subtract(&[1.0, 1.1]);
    assert!((result - (-0.1)).abs() < 1e-10);
}

#[test]
fn test_subtract_three_integers() {
    assert_eq!(umt_subtract(&[1.0, 1.0, 1.0]), -1.0);
}

#[test]
fn test_subtract_multiple() {
    assert_eq!(umt_subtract(&[5.0, 2.0, 1.0]), 2.0);
}

#[test]
fn test_subtract_four_integers() {
    assert_eq!(umt_subtract(&[10.0, 1.0, 1.0, 1.0]), 7.0);
}

#[test]
fn test_subtract_floating_point_precision_0_3_0_1() {
    let result = umt_subtract(&[0.3, 0.1]);
    assert!((result - 0.2).abs() < 1e-10);
}

#[test]
fn test_subtract_floating_point_precision_0_7_0_1() {
    let result = umt_subtract(&[0.7, 0.1]);
    assert!((result - 0.6).abs() < 1e-10);
}

#[test]
fn test_subtract_floating_point_precision_1_0_0_9() {
    let result = umt_subtract(&[1.0, 0.9]);
    assert!((result - 0.1).abs() < 1e-10);
}

#[test]
fn test_subtract_multiple_decimal_places() {
    let result = umt_subtract(&[0.12345, 0.00001]);
    assert!((result - 0.12344).abs() < 1e-10);
}

#[test]
fn test_subtract_three_decimals() {
    let result = umt_subtract(&[1.23456, 0.00001, 0.00002]);
    assert!((result - 1.23453).abs() < 1e-10);
}

#[test]
fn test_subtract_empty() {
    assert_eq!(umt_subtract(&[]), 0.0);
}

#[test]
fn test_subtract_single() {
    assert_eq!(umt_subtract(&[5.0]), 5.0);
}

use umt_rust::math::*;

#[test]
fn test_subtract_float() {
    let result = umt_subtract(&[0.3, 0.1]);
    assert!((result - 0.2).abs() < 1e-10);
}

#[test]
fn test_subtract_integers() {
    assert_eq!(umt_subtract(&[5.0, 3.0]), 2.0);
}

#[test]
fn test_subtract_negative() {
    assert_eq!(umt_subtract(&[-2.0, -3.0]), 1.0);
}

#[test]
fn test_subtract_result_negative() {
    let result = umt_subtract(&[0.1, 0.2]);
    assert!((result - (-0.1)).abs() < 1e-10);
}

#[test]
fn test_subtract_three_numbers() {
    let result = umt_subtract(&[1.0, 0.1, 0.2]);
    assert!((result - 0.7).abs() < 1e-10);
}
