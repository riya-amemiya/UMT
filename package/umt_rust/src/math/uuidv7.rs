use rand::RngExt;
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

    let mut bytes: [u8; 16] = rand::rng().random();

    // Timestamp (48 bits)
    for (i, byte) in bytes.iter_mut().enumerate().take(6) {
        *byte = ((unix_ts_ms >> ((5 - i) * 8)) & 0xff) as u8;
    }

    // Version (4 bits) + 4 random bits
    bytes[6] = 0x70 | (bytes[6] & 0x0f);

    // Variant (2 bits) + 6 random bits
    bytes[8] = 0x80 | (bytes[8] & 0x3f);

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
