use num_bigint::BigUint;
use num_traits::Zero;

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

    // Count leading zeros
    let leading_zeros = bytes.iter().take_while(|&&b| b == 0).count();

    // Convert bytes to big integer
    let big_number = BigUint::from_bytes_be(bytes);
    if big_number.is_zero() {
        return "1".repeat(leading_zeros);
    }

    // Convert to Base58
    let digits = big_number.to_radix_be(58);

    // Add leading '1's for leading zeros
    let mut encoded = String::with_capacity(leading_zeros + digits.len());
    encoded.extend(std::iter::repeat_n('1', leading_zeros));
    encoded.extend(
        digits
            .iter()
            .map(|&digit| char::from(ALPHABET[usize::from(digit)])),
    );
    encoded
}
