use umt_rust::math::umt_addition;

#[test]
fn test_addition_two_integers() {
    assert_eq!(umt_addition(&[2.0, 3.0]), 5.0);
}

#[test]
fn test_addition_two_negative_integers() {
    assert_eq!(umt_addition(&[-2.0, -3.0]), -5.0);
}

#[test]
fn test_addition_mixed_sign() {
    assert_eq!(umt_addition(&[2.0, -3.0]), -1.0);
}

#[test]
fn test_addition_two_decimals() {
    let result = umt_addition(&[0.1, 0.2]);
    assert!((result - 0.3).abs() < 1e-10);
}

#[test]
fn test_addition_two_negative_decimals() {
    let result = umt_addition(&[-0.1, -0.2]);
    assert!((result - (-0.3)).abs() < 1e-10);
}

#[test]
fn test_addition_mixed_sign_decimals() {
    let result = umt_addition(&[0.1, -0.2]);
    assert!((result - (-0.1)).abs() < 1e-10);
}

#[test]
fn test_addition_integer_and_decimal() {
    let result = umt_addition(&[2.0, 0.3]);
    assert!((result - 2.3).abs() < 1e-10);
}

#[test]
fn test_addition_negative_integer_and_decimal() {
    let result = umt_addition(&[-2.0, -0.3]);
    assert!((result - (-2.3)).abs() < 1e-10);
}

#[test]
fn test_addition_integer_minus_decimal() {
    let result = umt_addition(&[2.0, -0.3]);
    assert!((result - 1.7).abs() < 1e-10);
}

#[test]
fn test_addition_negative_integer_plus_decimal() {
    let result = umt_addition(&[-2.0, 0.3]);
    assert!((result - (-1.7)).abs() < 1e-10);
}

#[test]
fn test_addition_three_integers() {
    assert_eq!(umt_addition(&[1.0, 2.0, 3.0]), 6.0);
}

#[test]
fn test_addition_three_negative_integers() {
    assert_eq!(umt_addition(&[-1.0, -2.0, -3.0]), -6.0);
}

#[test]
fn test_addition_three_decimals() {
    let result = umt_addition(&[0.1, 0.2, 0.3]);
    assert!((result - 0.6).abs() < 1e-10);
}

#[test]
fn test_addition_three_mixed() {
    assert_eq!(umt_addition(&[2.0, -3.0, 1.0]), 0.0);
}

#[test]
fn test_addition_mixed_with_decimals() {
    let result = umt_addition(&[-2.0, 0.5, 1.5]);
    assert!((result - 0.0).abs() < 1e-10);
}

#[test]
fn test_addition_empty() {
    assert_eq!(umt_addition(&[]), 0.0);
}

#[test]
fn test_addition_single() {
    assert_eq!(umt_addition(&[5.0]), 5.0);
}
