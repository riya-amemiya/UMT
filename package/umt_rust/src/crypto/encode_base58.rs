use crate::internal::bigint::BigUint;

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

    // Count leading zeros
    let leading_zeros = bytes.iter().take_while(|&&b| b == 0).count();

    // Convert bytes to big integer
    let mut big_number = BigUint::from_bytes_be(bytes);

    // Convert to Base58
    let mut encoded = String::new();

    while !big_number.is_zero() {
        let (quotient, remainder) = big_number.div_mod_u32(58);
        encoded.insert(0, ALPHABET[remainder as usize] as char);
        big_number = quotient;
    }

    // Add leading '1's for leading zeros
    let leading_ones = "1".repeat(leading_zeros);
    leading_ones + &encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_simple_string() {
        assert_eq!(umt_encode_base58("Hello"), "9Ajdvzr");
    }

    #[test]
    fn test_encode_empty_string() {
        assert_eq!(umt_encode_base58(""), "");
    }

    #[test]
    fn test_encode_single_characters() {
        assert_eq!(umt_encode_base58("a"), "2g");
        assert_eq!(umt_encode_base58("b"), "2h");
        assert_eq!(umt_encode_base58("c"), "2i");
    }

    #[test]
    fn test_encode_byte_slice() {
        let bytes: &[u8] = &[72, 101, 108, 108, 111]; // "Hello"
        assert_eq!(umt_encode_base58(bytes), "9Ajdvzr");
    }

    #[test]
    fn test_encode_with_leading_zeros() {
        let bytes: &[u8] = &[0, 0, 72, 101, 108, 108, 111];
        assert_eq!(umt_encode_base58(bytes), "119Ajdvzr");
    }

    #[test]
    fn test_encode_longer_text() {
        let text = "The quick brown fox jumps over the lazy dog";
        assert_eq!(
            umt_encode_base58(text),
            "7DdiPPYtxLjCD3wA1po2rvZHTDYjkZYiEtazrfiwJcwnKCizhGFhBGHeRdx"
        );
    }

    #[test]
    fn test_encode_special_characters() {
        assert_eq!(umt_encode_base58("こんにちは"), "7NAasPYBzpyEe5hmwr1KL");
    }

    #[test]
    fn test_encode_binary_data() {
        let bytes: &[u8] = &[255, 254, 253, 252, 251];
        assert_eq!(umt_encode_base58(bytes), "Vt9aq46");
    }
}
