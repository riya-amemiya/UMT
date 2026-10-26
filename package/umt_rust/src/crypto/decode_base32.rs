/// Error type for Base32 decoding failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base32Error {
    pub message: String,
}

impl std::fmt::Display for Base32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Base32Error {}

const fn base32_decode_table() -> [u8; 256] {
    let mut table = [0xFFu8; 256];
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut index = 0;
    while index < alphabet.len() {
        table[alphabet[index] as usize] = index as u8;
        index += 1;
    }
    table
}

const BASE32_DECODE_TABLE: [u8; 256] = base32_decode_table();

/// Decodes a Base32 string to bytes.
///
/// Uses the standard RFC 4648 Base32 alphabet (A-Z, 2-7).
/// Padding characters ('=') are handled automatically.
///
/// # Arguments
///
/// * `input` - A Base32 encoded string.
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - The decoded bytes.
/// * `Err(Base32Error)` - If the input contains invalid Base32 characters.
///
/// # Examples
///
/// ```
/// use umt_rust::crypto::umt_decode_base32;
///
/// let result = umt_decode_base32("JBSWY3DP").unwrap();
/// assert_eq!(String::from_utf8(result).unwrap(), "Hello");
/// ```
pub fn umt_decode_base32(input: &str) -> Result<Vec<u8>, Base32Error> {
    let mut result: Vec<u8> = Vec::with_capacity(input.len().div_ceil(8).saturating_mul(5));
    let mut buffer: u32 = 0;
    let mut buffer_length: u32 = 0;

    for character in input.chars() {
        if character == '=' {
            continue;
        }

        let value = BASE32_DECODE_TABLE
            .get(character as usize)
            .copied()
            .unwrap_or(0xFF);
        if value > 31 {
            return Err(Base32Error {
                message: format!("Invalid base32 character: {}", character),
            });
        }

        buffer = (buffer << 5) | u32::from(value);
        buffer_length += 5;

        if buffer_length >= 8 {
            buffer_length -= 8;
            result.push(((buffer >> buffer_length) & 0xff) as u8);
        }
    }

    Ok(result)
}
