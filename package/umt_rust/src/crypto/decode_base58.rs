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

const fn base58_decode_table() -> [u8; 256] {
    let mut table = [0xFFu8; 256];
    let alphabet = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let mut index = 0;
    while index < alphabet.len() {
        table[alphabet[index] as usize] = index as u8;
        index += 1;
    }
    table
}

const BASE58_DECODE_TABLE: [u8; 256] = base58_decode_table();

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
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let mut digits: Vec<u8> = Vec::with_capacity(input.len());
    for char in input.chars() {
        let value = BASE58_DECODE_TABLE
            .get(char as usize)
            .copied()
            .unwrap_or(0xFF);
        if value == 0xFF {
            return Err(Base58Error {
                message: format!("Invalid base58 character: {}", char),
            });
        }
        digits.push(value);
    }

    // Convert Base58 string to big integer
    let big_number = BigUint::from_radix_be(&digits, 58).unwrap_or_default();

    // Count leading '1's (which represent leading zeros)
    let leading_ones = digits.iter().take_while(|&&digit| digit == 0).count();

    // Prepend leading zeros
    let mut result = vec![0u8; leading_ones];
    if !big_number.is_zero() {
        result.extend(big_number.to_bytes_be());
    }

    Ok(result)
}
