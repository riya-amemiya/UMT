use umt_rust::math::{umt_to_base_n, umt_to_binary, umt_to_hex};

#[test]
fn test_to_base_n_binary_1() {
    assert_eq!(umt_to_base_n(1, 2), "1");
}

#[test]
fn test_to_base_n_binary_2() {
    assert_eq!(umt_to_base_n(2, 2), "10");
}

#[test]
fn test_to_base_n_binary_3() {
    assert_eq!(umt_to_base_n(3, 2), "11");
}

#[test]
fn test_to_base_n_binary_4() {
    assert_eq!(umt_to_base_n(4, 2), "100");
}

#[test]
fn test_to_base_n_binary_5() {
    assert_eq!(umt_to_base_n(5, 2), "101");
}

#[test]
fn test_to_base_n_binary_6() {
    assert_eq!(umt_to_base_n(6, 2), "110");
}

#[test]
fn test_to_base_n_binary_112() {
    assert_eq!(umt_to_base_n(112, 2), "1110000");
}

#[test]
fn test_to_base_n_base4_7() {
    assert_eq!(umt_to_base_n(7, 4), "13");
}

#[test]
fn test_to_base_n_base4_8() {
    assert_eq!(umt_to_base_n(8, 4), "20");
}

#[test]
fn test_to_base_n_base4_9() {
    assert_eq!(umt_to_base_n(9, 4), "21");
}

#[test]
fn test_to_base_n_base4_112() {
    assert_eq!(umt_to_base_n(112, 4), "1300");
}

#[test]
fn test_to_base_n_octal_7() {
    assert_eq!(umt_to_base_n(7, 8), "7");
}

#[test]
fn test_to_base_n_octal_8() {
    assert_eq!(umt_to_base_n(8, 8), "10");
}

#[test]
fn test_to_base_n_octal_9() {
    assert_eq!(umt_to_base_n(9, 8), "11");
}

#[test]
fn test_to_base_n_octal_112() {
    assert_eq!(umt_to_base_n(112, 8), "160");
}

#[test]
fn test_to_base_n_hex_10() {
    assert_eq!(umt_to_base_n(10, 16), "a");
}

#[test]
fn test_to_base_n_hex_15() {
    assert_eq!(umt_to_base_n(15, 16), "f");
}

#[test]
fn test_to_base_n_hex_16() {
    assert_eq!(umt_to_base_n(16, 16), "10");
}

#[test]
fn test_to_base_n_hex_255() {
    assert_eq!(umt_to_base_n(255, 16), "ff");
}

#[test]
fn test_to_binary_helper() {
    assert_eq!(umt_to_binary(10), "1010");
}

#[test]
fn test_to_hex_helper() {
    assert_eq!(umt_to_hex(255), "ff");
}

use umt_rust::math::*;

#[test]
fn test_to_base_n_binary() {
    assert_eq!(umt_to_base_n(10, 2), "1010");
}

#[test]
fn test_to_base_n_hex() {
    assert_eq!(umt_to_base_n(15, 16), "f");
    assert_eq!(umt_to_base_n(255, 16), "ff");
}

#[test]
fn test_to_base_n_invalid_radix() {
    assert_eq!(umt_to_base_n(10, 1), "Invalid radix");
    assert_eq!(umt_to_base_n(10, 37), "Invalid radix");
}

#[test]
fn test_to_base_n_negative() {
    assert_eq!(umt_to_base_n(-10, 2), "-1010");
}

#[test]
fn test_to_base_n_octal() {
    assert_eq!(umt_to_base_n(8, 8), "10");
}

#[test]
fn test_to_base_n_zero() {
    assert_eq!(umt_to_base_n(0, 2), "0");
    assert_eq!(umt_to_base_n(0, 16), "0");
}

#[test]
fn test_to_binary() {
    assert_eq!(umt_to_binary(10), "1010");
}

#[test]
fn test_to_hex() {
    assert_eq!(umt_to_hex(255), "ff");
}
