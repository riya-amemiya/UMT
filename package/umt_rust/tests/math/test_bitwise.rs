use umt_rust::math::{RotateDirection, umt_bitwise, umt_bitwise_left};

#[test]
fn test_bitwise_rotate_8_left() {
    let result = umt_bitwise(0x12345678, 8, RotateDirection::Left);
    assert_eq!(result, 0x34567812);
}

#[test]
fn test_bitwise_rotate_16_left() {
    let result = umt_bitwise(0x12345678, 16, RotateDirection::Left);
    assert_eq!(result, 0x56781234);
}

#[test]
fn test_bitwise_rotate_24_left() {
    let result = umt_bitwise(0x12345678, 24, RotateDirection::Left);
    assert_eq!(result, 0x78123456);
}

#[test]
fn test_bitwise_rotate_32_left() {
    let result = umt_bitwise(0x12345678, 32, RotateDirection::Left);
    assert_eq!(result, 0x12345678);
}

#[test]
fn test_bitwise_rotate_0_left() {
    let result = umt_bitwise(0x12345678, 0, RotateDirection::Left);
    assert_eq!(result, 0x12345678);
}

#[test]
fn test_bitwise_negative_count_left() {
    let result = umt_bitwise(0x12345678, -8, RotateDirection::Left);
    assert_eq!(result, 0x78123456);
}

#[test]
fn test_bitwise_over_32_left() {
    let result = umt_bitwise(0x12345678, 40, RotateDirection::Left);
    assert_eq!(result, 0x34567812);
}

#[test]
fn test_bitwise_rotate_8_right() {
    let result = umt_bitwise(0x12345678, 8, RotateDirection::Right);
    assert_eq!(result, 0x78123456);
}

#[test]
fn test_bitwise_rotate_16_right() {
    let result = umt_bitwise(0x12345678, 16, RotateDirection::Right);
    assert_eq!(result, 0x56781234);
}

#[test]
fn test_bitwise_rotate_24_right() {
    let result = umt_bitwise(0x12345678, 24, RotateDirection::Right);
    assert_eq!(result, 0x34567812);
}

#[test]
fn test_bitwise_rotate_32_right() {
    let result = umt_bitwise(0x12345678, 32, RotateDirection::Right);
    assert_eq!(result, 0x12345678);
}

#[test]
fn test_bitwise_rotate_0_right() {
    let result = umt_bitwise(0x12345678, 0, RotateDirection::Right);
    assert_eq!(result, 0x12345678);
}

#[test]
fn test_bitwise_over_32_right() {
    let result = umt_bitwise(0x12345678, 40, RotateDirection::Right);
    assert_eq!(result, 0x78123456);
}

#[test]
fn test_bitwise_left_helper() {
    let result = umt_bitwise_left(0x12345678, 8);
    assert_eq!(result, 0x34567812);
}
