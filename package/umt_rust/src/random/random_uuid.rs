use rand::RngExt;

const DIGITS: &[u8] = b"0123456789abcdef";

fn bytes_to_uuid(bytes: &[u8; 16]) -> String {
    let mut result = String::with_capacity(36);
    for (index, byte) in bytes.iter().enumerate() {
        if index == 4 || index == 6 || index == 8 || index == 10 {
            result.push('-');
        }
        result.push(DIGITS[(byte >> 4) as usize] as char);
        result.push(DIGITS[(byte & 0xf) as usize] as char);
    }
    result
}

/// Generates a UUID v4 string in canonical 8-4-4-4-12 hex form.
///
/// This mirrors the TypeScript `randomUUID` fallback path, which fills 16
/// random bytes, sets the version (4) and variant (8/9/a/b) bits, and formats
/// the result.
///
/// # Returns
///
/// A UUID v4 string, e.g. `"1b9d6bcd-bbfd-4b2d-9b5d-ab8dfbbd4bed"`.
///
/// # Examples
///
/// ```
/// use umt_rust::random::umt_random_uuid;
///
/// let id = umt_random_uuid();
/// assert_eq!(id.len(), 36);
/// assert_eq!(&id[14..15], "4"); // version 4
/// ```
pub fn umt_random_uuid() -> String {
    let mut rng = rand::rng();
    let mut bytes = [0u8; 16];
    for byte in bytes.iter_mut() {
        *byte = rng.random();
    }
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    bytes_to_uuid(&bytes)
}
