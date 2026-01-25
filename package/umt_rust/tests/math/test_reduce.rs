use umt_rust::math::umt_reduce;

#[test]
fn test_reduce_simple() {
    let result = umt_reduce(2, 4).unwrap();
    assert_eq!(result.x, 1);
    assert_eq!(result.y, 2);
    assert_eq!(result.gcd, 2);
}

#[test]
fn test_reduce_3_6() {
    let result = umt_reduce(3, 6).unwrap();
    assert_eq!(result.x, 1);
    assert_eq!(result.y, 2);
    assert_eq!(result.gcd, 3);
}

#[test]
fn test_reduce_8_12() {
    let result = umt_reduce(8, 12).unwrap();
    assert_eq!(result.x, 2);
    assert_eq!(result.y, 3);
    assert_eq!(result.gcd, 4);
}

#[test]
fn test_reduce_already_reduced() {
    let result = umt_reduce(3, 4).unwrap();
    assert_eq!(result.x, 3);
    assert_eq!(result.y, 4);
    assert_eq!(result.gcd, 1);
}

#[test]
fn test_reduce_7_9() {
    let result = umt_reduce(7, 9).unwrap();
    assert_eq!(result.x, 7);
    assert_eq!(result.y, 9);
    assert_eq!(result.gcd, 1);
}

#[test]
fn test_reduce_denominator_1() {
    let result = umt_reduce(5, 1).unwrap();
    assert_eq!(result.x, 5);
    assert_eq!(result.y, 1);
    assert_eq!(result.gcd, 1);
}

#[test]
fn test_reduce_numerator_1() {
    let result = umt_reduce(1, 7).unwrap();
    assert_eq!(result.x, 1);
    assert_eq!(result.y, 7);
    assert_eq!(result.gcd, 1);
}

#[test]
fn test_reduce_zero_numerator() {
    let result = umt_reduce(0, 5).unwrap();
    assert_eq!(result.x, 0);
    assert_eq!(result.y, 1);
    assert_eq!(result.gcd, 5);
}

#[test]
fn test_reduce_zero_denominator() {
    assert!(umt_reduce(5, 0).is_none());
}

#[test]
fn test_reduce_both_zero() {
    assert!(umt_reduce(0, 0).is_none());
}

#[test]
fn test_reduce_negative_numerator() {
    let result = umt_reduce(-6, 8).unwrap();
    assert_eq!(result.x, -3);
    assert_eq!(result.y, 4);
    assert_eq!(result.gcd, 2);
}

#[test]
fn test_reduce_negative_denominator() {
    let result = umt_reduce(6, -8).unwrap();
    assert_eq!(result.x, -3);
    assert_eq!(result.y, 4);
    assert_eq!(result.gcd, 2);
}

#[test]
fn test_reduce_both_negative() {
    let result = umt_reduce(-6, -8).unwrap();
    assert_eq!(result.x, 3);
    assert_eq!(result.y, 4);
    assert_eq!(result.gcd, 2);
}

#[test]
fn test_reduce_large_numbers() {
    let result = umt_reduce(1000, 2500).unwrap();
    assert_eq!(result.x, 2);
    assert_eq!(result.y, 5);
    assert_eq!(result.gcd, 500);
}

#[test]
fn test_reduce_24680_12340() {
    let result = umt_reduce(24680, 12340).unwrap();
    assert_eq!(result.x, 2);
    assert_eq!(result.y, 1);
    assert_eq!(result.gcd, 12340);
}
