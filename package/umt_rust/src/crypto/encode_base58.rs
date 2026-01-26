/// Encodes a byte slice or string to Base58 format.
///
/// Uses the Bitcoin Base58 alphabet which excludes ambiguous characters
/// (0, O, I, l) to improve readability.
///
/// # Arguments
///
/// * `input` - The bytes to encode (can be a string or byte slice).
///
/// # Returns
///
/// A Base58 encoded string.
///
/// # Examples
///
/// ```
/// use umt_rust::crypto::umt_encode_base58;
///
/// assert_eq!(umt_encode_base58("Hello"), "9Ajdvzr");
/// ```
pub fn umt_encode_base58<T: AsRef<[u8]>>(input: T) -> String {
    const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    let bytes = input.as_ref();

    if bytes.is_empty() {
        return String::new();
    }

    let leading_zeros = bytes.iter().take_while(|&&b| b == 0).count();

    // Work directly on a big-endian byte array (base-256 number)
    let mut num = bytes.to_vec();
    let mut encoded = Vec::new();

    while num.iter().any(|&d| d != 0) {
        let mut remainder: u32 = 0;
        for d in num.iter_mut() {
            let acc = remainder * 256 + *d as u32;
            *d = (acc / 58) as u8;
            remainder = acc % 58;
        }
        encoded.push(ALPHABET[remainder as usize]);
    }

    encoded.reverse();

    let mut result = String::with_capacity(leading_zeros + encoded.len());
    for _ in 0..leading_zeros {
        result.push('1');
    }
    for &b in &encoded {
        result.push(b as char);
    }
    result
}
