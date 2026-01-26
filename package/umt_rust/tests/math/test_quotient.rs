use umt_rust::math::umt_quotient;

#[test]
fn test_quotient_exact() {
    let (q, r) = umt_quotient(4.0, 2.0);
    assert_eq!(q, 2.0);
    assert_eq!(r, 0.0);
}

#[test]
fn test_quotient_10_5() {
    let (q, r) = umt_quotient(10.0, 5.0);
    assert_eq!(q, 2.0);
    assert_eq!(r, 0.0);
}

#[test]
fn test_quotient_100_10() {
    let (q, r) = umt_quotient(100.0, 10.0);
    assert_eq!(q, 10.0);
    assert_eq!(r, 0.0);
}

#[test]
fn test_quotient_with_remainder() {
    let (q, r) = umt_quotient(5.0, 2.0);
    assert_eq!(q, 2.0);
    assert_eq!(r, 1.0);
}

#[test]
fn test_quotient_7_3() {
    let (q, r) = umt_quotient(7.0, 3.0);
    assert_eq!(q, 2.0);
    assert_eq!(r, 1.0);
}

#[test]
fn test_quotient_10_3() {
    let (q, r) = umt_quotient(10.0, 3.0);
    assert_eq!(q, 3.0);
    assert_eq!(r, 1.0);
}

#[test]
fn test_quotient_divide_by_1() {
    let (q, r) = umt_quotient(5.0, 1.0);
    assert_eq!(q, 5.0);
    assert_eq!(r, 0.0);
}

#[test]
fn test_quotient_zero_dividend() {
    let (q, r) = umt_quotient(0.0, 1.0);
    assert_eq!(q, 0.0);
    assert_eq!(r, 0.0);
}

#[test]
fn test_quotient_smaller_dividend() {
    let (q, r) = umt_quotient(2.0, 5.0);
    assert_eq!(q, 0.0);
    assert_eq!(r, 2.0);
}

#[test]
fn test_quotient_1_10() {
    let (q, r) = umt_quotient(1.0, 10.0);
    assert_eq!(q, 0.0);
    assert_eq!(r, 1.0);
}

#[test]
fn test_quotient_negative_dividend() {
    let (q, r) = umt_quotient(-5.0, 2.0);
    assert_eq!(q, -2.0);
    assert_eq!(r, -1.0);
}

#[test]
fn test_quotient_negative_divisor() {
    let (q, r) = umt_quotient(5.0, -2.0);
    assert_eq!(q, -2.0);
    assert_eq!(r, 1.0);
}

#[test]
fn test_quotient_both_negative() {
    let (q, r) = umt_quotient(-5.0, -2.0);
    assert_eq!(q, 2.0);
    assert_eq!(r, -1.0);
}

#[test]
fn test_quotient_large() {
    let (q, r) = umt_quotient(1000.0, 3.0);
    assert_eq!(q, 333.0);
    assert_eq!(r, 1.0);
}

#[test]
fn test_quotient_9999_10() {
    let (q, r) = umt_quotient(9999.0, 10.0);
    assert_eq!(q, 999.0);
    assert_eq!(r, 9.0);
}

use umt_rust::math::*;

#[test]
fn test_quotient_basic() {
    let (q, r) = umt_quotient(5.0, 2.0);
    assert_eq!(q, 2.0);
    assert_eq!(r, 1.0);
}

#[test]
fn test_quotient_larger() {
    let (q, r) = umt_quotient(10.0, 3.0);
    assert_eq!(q, 3.0);
    assert_eq!(r, 1.0);
}

#[test]
fn test_quotient_negative() {
    let (q, r) = umt_quotient(-5.0, 2.0);
    assert_eq!(q, -2.0);
    assert_eq!(r, -1.0);
}
