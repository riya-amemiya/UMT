use umt_rust::math::umt_multiplication;

#[test]
fn test_multiplication_two_positive() {
    assert_eq!(umt_multiplication(&[2.0, 3.0]), 6.0);
    assert_eq!(umt_multiplication(&[10.0, 20.0]), 200.0);
}

#[test]
fn test_multiplication_multiple_positive() {
    assert_eq!(umt_multiplication(&[2.0, 3.0, 4.0]), 24.0);
    assert_eq!(umt_multiplication(&[1.0, 2.0, 3.0, 4.0, 5.0]), 120.0);
}

#[test]
fn test_multiplication_decimals() {
    let result = umt_multiplication(&[0.1, 0.2]);
    assert!((result - 0.02).abs() < 1e-10);
}

#[test]
fn test_multiplication_three_decimals() {
    let result = umt_multiplication(&[0.1, 0.2, 0.3]);
    assert!((result - 0.006).abs() < 1e-10);
}

#[test]
fn test_multiplication_complex_decimals() {
    let result = umt_multiplication(&[1.23, 4.56]);
    assert!((result - 5.6088).abs() < 1e-10);
}

#[test]
fn test_multiplication_mixed() {
    assert_eq!(umt_multiplication(&[0.5, 2.0]), 1.0);
    assert_eq!(umt_multiplication(&[0.1, 10.0]), 1.0);
    assert_eq!(umt_multiplication(&[1.5, 2.0, 3.0]), 9.0);
}

#[test]
fn test_multiplication_negative() {
    assert_eq!(umt_multiplication(&[-2.0, 3.0]), -6.0);
    assert_eq!(umt_multiplication(&[2.0, -3.0]), -6.0);
    assert_eq!(umt_multiplication(&[-2.0, -3.0]), 6.0);
}

#[test]
fn test_multiplication_with_zero() {
    assert_eq!(umt_multiplication(&[0.0, 5.0]), 0.0);
    assert_eq!(umt_multiplication(&[5.0, 0.0]), 0.0);
    assert_eq!(umt_multiplication(&[0.0, 0.0]), 0.0);
}

#[test]
fn test_multiplication_with_one() {
    assert_eq!(umt_multiplication(&[1.0, 5.0]), 5.0);
    assert_eq!(umt_multiplication(&[5.0, 1.0]), 5.0);
    assert_eq!(umt_multiplication(&[1.0, 1.0]), 1.0);
}

#[test]
fn test_multiplication_very_small() {
    let result = umt_multiplication(&[0.0001, 0.0001]);
    assert!((result - 0.00000001).abs() < 1e-15);
}

#[test]
fn test_multiplication_empty() {
    assert_eq!(umt_multiplication(&[]), 1.0);
}

#[test]
fn test_multiplication_single() {
    assert_eq!(umt_multiplication(&[5.0]), 5.0);
}
