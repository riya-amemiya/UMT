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

    if bytes.is_empty() {
        return String::new();
    }

    // Count leading zeros
    let leading_zeros = bytes.iter().take_while(|&&b| b == 0).count();

    // Convert bytes to big integer
    let mut big_number = BigUint::from_bytes_be(bytes);

    // Convert to Base58
    let mut encoded = String::new();
    let fifty_eight = BigUint::from(58u32);

    while !big_number.is_zero() {
        let remainder = &big_number % &fifty_eight;
        let digit = remainder.iter_u32_digits().next().unwrap_or(0) as usize;
        encoded.insert(0, ALPHABET[digit] as char);
        big_number /= &fifty_eight;
    }

    // Add leading '1's for leading zeros
    let leading_ones = "1".repeat(leading_zeros);
    leading_ones + &encoded
}
