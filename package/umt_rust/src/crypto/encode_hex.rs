/// Encodes a byte slice or string to lowercase hexadecimal.
///
/// # Arguments
///
/// * `input` - The bytes to encode (can be a string or byte slice).
///
/// # Returns
///
/// A lowercase hexadecimal string with two characters per byte.
///
/// # Examples
///
/// ```
/// use umt_rust::crypto::umt_encode_hex;
///
/// assert_eq!(umt_encode_hex("Hello"), "48656c6c6f");
/// assert_eq!(umt_encode_hex([0u8, 255]), "00ff");
/// ```
pub fn umt_encode_hex<T: AsRef<[u8]>>(input: T) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";

    let bytes = input.as_ref();
    let mut result = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        result.push(char::from(DIGITS[usize::from(byte >> 4)]));
        result.push(char::from(DIGITS[usize::from(byte & 0x0F)]));
    }
    result
}
