use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

/// Generates a UUID v7 (Universally Unique Identifier version 7).
///
/// UUID v7 is time-ordered and contains:
/// - 48 bits of Unix timestamp in milliseconds
/// - 74 bits of random data
/// - 4 bits of version (7)
/// - 2 bits of variant (2)
///
/// This implementation follows the UUID v7 draft specification.
///
/// # Returns
///
/// A UUID v7 string in the format xxxxxxxx-xxxx-7xxx-8xxx-xxxxxxxxxxxx
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_uuidv7;
///
/// let id = umt_uuidv7();
/// // e.g. "018d6e78-e1e5-7c3c-8bf9-ae5942f2ba1c"
/// assert_eq!(id.len(), 36);
/// assert_eq!(&id[14..15], "7"); // Version 7
/// ```
pub fn umt_uuidv7() -> String {
    const DIGITS: &[u8] = b"0123456789abcdef";

    let unix_ts_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64;

    let mut rng = rand::rng();
    let rand_a: u16 = rng.random_range(0..0xFFF);
    let rand_b_hi: u32 = rng.random_range(0..0x3FFFFFFF);
    let rand_b_lo: u32 = rng.random();

    let mut bytes = [0u8; 16];

    // Timestamp (48 bits)
    for (i, byte) in bytes.iter_mut().enumerate().take(6) {
        *byte = ((unix_ts_ms >> ((5 - i) * 8)) & 0xff) as u8;
    }

    // Version (4 bits) + rand_a (12 bits)
    bytes[6] = 0x70 | ((rand_a >> 8) as u8);
    bytes[7] = (rand_a & 0xff) as u8;

    // Variant (2 bits) + rand_b_hi (30 bits)
    bytes[8] = 0x80 | ((rand_b_hi >> 24) as u8 & 0x3f);
    bytes[9] = ((rand_b_hi >> 16) & 0xff) as u8;
    bytes[10] = ((rand_b_hi >> 8) & 0xff) as u8;
    bytes[11] = (rand_b_hi & 0xff) as u8;

    // rand_b_lo (32 bits)
    bytes[12] = ((rand_b_lo >> 24) & 0xff) as u8;
    bytes[13] = ((rand_b_lo >> 16) & 0xff) as u8;
    bytes[14] = ((rand_b_lo >> 8) & 0xff) as u8;
    bytes[15] = (rand_b_lo & 0xff) as u8;

    let mut uuid = String::with_capacity(36);
    for (i, byte) in bytes.iter().enumerate() {
        uuid.push(DIGITS[(byte >> 4) as usize] as char);
        uuid.push(DIGITS[(byte & 0xf) as usize] as char);
        if i == 3 || i == 5 || i == 7 || i == 9 {
            uuid.push('-');
        }
    }

    uuid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuidv7_format() {
        let id = umt_uuidv7();
        assert_eq!(id.len(), 36);
        assert_eq!(&id[8..9], "-");
        assert_eq!(&id[13..14], "-");
        assert_eq!(&id[18..19], "-");
        assert_eq!(&id[23..24], "-");
    }

    #[test]
    fn test_uuidv7_version() {
        let id = umt_uuidv7();
        assert_eq!(&id[14..15], "7");
    }

    #[test]
    fn test_uuidv7_variant() {
        let id = umt_uuidv7();
        let variant_char = id.chars().nth(19).unwrap();
        assert!(
            variant_char == '8'
                || variant_char == '9'
                || variant_char == 'a'
                || variant_char == 'b'
        );
    }

    #[test]
    fn test_uuidv7_uniqueness() {
        let id1 = umt_uuidv7();
        let id2 = umt_uuidv7();
        assert_ne!(id1, id2);
    }
}
