/// Direction for bit rotation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RotateDirection {
    Left,
    Right,
}

/// Performs bit rotation on a 32-bit integer.
///
/// This function rotates bits of a 32-bit integer by k positions in the specified direction.
///
/// # Arguments
///
/// * `x` - 32-bit integer to rotate.
/// * `k` - Number of bits to rotate.
/// * `direction` - Direction of rotation (Left or Right).
///
/// # Returns
///
/// The result after rotating k bits in the specified direction.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_bitwise;
/// use umt_rust::math::RotateDirection;
///
/// let result_left = umt_bitwise(0x12345678, 8, RotateDirection::Left);
/// // result_left is 0x34567812
///
/// let result_right = umt_bitwise(0x12345678, 8, RotateDirection::Right);
/// // result_right is 0x78123456
/// ```
#[inline]
pub fn umt_bitwise(x: u32, k: i32, direction: RotateDirection) -> u32 {
    let rotation = k.rem_euclid(32);
    match direction {
        RotateDirection::Left => x.rotate_left(rotation as u32),
        RotateDirection::Right => x.rotate_right(rotation as u32),
    }
}

/// Performs left bit rotation on a 32-bit integer (default direction).
///
/// # Arguments
///
/// * `x` - 32-bit integer to rotate.
/// * `k` - Number of bits to rotate.
///
/// # Returns
///
/// The result after rotating k bits to the left.
#[inline]
pub fn umt_bitwise_left(x: u32, k: i32) -> u32 {
    umt_bitwise(x, k, RotateDirection::Left)
}
