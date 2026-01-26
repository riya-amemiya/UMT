/// Encodes a byte slice or string to Base32 format.
///
/// Uses the standard RFC 4648 Base32 alphabet (A-Z, 2-7) with padding.
///
/// # Arguments
///
/// * `input` - The bytes to encode (can be a string or byte slice).
///
/// # Returns
///
/// A Base32 encoded string with padding.
///
/// # Examples
///
/// ```
/// use umt_rust::crypto::umt_encode_base32;
///
/// assert_eq!(umt_encode_base32("Hello"), "JBSWY3DP");
/// assert_eq!(umt_encode_base32("f"), "MY======");
/// ```
pub fn umt_encode_base32<T: AsRef<[u8]>>(input: T) -> String {
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    let bytes = input.as_ref();

    if bytes.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut buffer: u32 = 0;
    let mut buffer_length: u32 = 0;

    for &byte in bytes {
        buffer = (buffer << 8) | u32::from(byte);
        buffer_length += 8;

        while buffer_length >= 5 {
            buffer_length -= 5;
            let index = ((buffer >> buffer_length) & 0x1f) as usize;
            result.push(ALPHABET[index] as char);
        }
    }

    if buffer_length > 0 {
        let index = ((buffer << (5 - buffer_length)) & 0x1f) as usize;
        result.push(ALPHABET[index] as char);
    }

    // Add padding
    let padding_length = (8 - (result.len() % 8)) % 8;
    result.push_str(&"=".repeat(padding_length));

    result
}
