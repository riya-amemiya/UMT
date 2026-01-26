use num_bigint::BigUint;
use num_traits::Zero;

/// Error type for Base58 decoding failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base58Error {
    pub message: String,
}

impl std::fmt::Display for Base58Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Base58Error {}

/// Decodes a Base58 string to bytes.
///
/// Uses the Bitcoin Base58 alphabet which excludes ambiguous characters
/// (0, O, I, l) to improve readability.
///
/// # Arguments
///
/// * `input` - A Base58 encoded string.
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - The decoded bytes.
/// * `Err(Base58Error)` - If the input contains invalid Base58 characters.
///
/// # Examples
///
/// ```
/// use umt_rust::crypto::umt_decode_base58;
///
/// let result = umt_decode_base58("9Ajdvzr").unwrap();
/// assert_eq!(String::from_utf8(result).unwrap(), "Hello");
/// ```
pub fn umt_decode_base58(input: &str) -> Result<Vec<u8>, Base58Error> {
    const ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    if input.is_empty() {
        return Ok(Vec::new());
    }

    // Convert Base58 string to big integer
    let mut big_number = BigUint::zero();
    let fifty_eight = BigUint::from(58u32);

    for char in input.chars() {
        let value = match ALPHABET.find(char) {
            Some(v) => v,
            None => {
                return Err(Base58Error {
                    message: format!("Invalid base58 character: {}", char),
                });
            }
        };
        big_number = big_number * &fifty_eight + BigUint::from(value);
    }

    // Convert big integer to bytes
    let bytes = big_number.to_bytes_be();

    // Count leading '1's (which represent leading zeros)
    let leading_ones = input.chars().take_while(|&c| c == '1').count();

    // Prepend leading zeros
    let mut result = vec![0u8; leading_ones];
    if !big_number.is_zero() {
        result.extend(bytes);
    }

    Ok(result)
}
