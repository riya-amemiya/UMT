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

    let leading_ones = input.chars().take_while(|&c| c == '1').count();

    // Accumulate as big-endian base-256 number
    let mut bytes: Vec<u8> = vec![0];

    for ch in input.chars() {
        let val = match ALPHABET.find(ch) {
            Some(v) => v as u32,
            None => {
                return Err(Base58Error {
                    message: format!("Invalid base58 character: {}", ch),
                });
            }
        };

        // bytes = bytes * 58 + val
        let mut carry = val;
        for b in bytes.iter_mut().rev() {
            let acc = *b as u32 * 58 + carry;
            *b = (acc & 0xFF) as u8;
            carry = acc >> 8;
        }
        while carry > 0 {
            bytes.insert(0, (carry & 0xFF) as u8);
            carry >>= 8;
        }
    }

    // Remove leading zeros from computed bytes, then prepend leading_ones as 0x00 bytes
    let data_start = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    let mut result = vec![0u8; leading_ones];
    result.extend_from_slice(&bytes[data_start..]);

    Ok(result)
}
