use std::collections::HashSet;
use umt_rust::math::umt_uuidv7;

fn is_valid_uuidv7(s: &str) -> bool {
    if s.len() != 36 { return false; }
    let bytes = s.as_bytes();
    // Check dashes at positions 8, 13, 18, 23
    if bytes[8] != b'-' || bytes[13] != b'-' || bytes[18] != b'-' || bytes[23] != b'-' { return false; }
    // Check version '7' at position 14
    if bytes[14] != b'7' { return false; }
    // Check variant at position 19 (must be 8, 9, a, or b)
    let variant = bytes[19];
    if variant != b'8' && variant != b'9' && variant != b'a' && variant != b'b' { return false; }
    // Check all other chars are lowercase hex
    for (i, &b) in bytes.iter().enumerate() {
        if i == 8 || i == 13 || i == 18 || i == 23 { continue; }
        if !b.is_ascii_hexdigit() || (b.is_ascii_alphabetic() && !b.is_ascii_lowercase()) { return false; }
    }
    true
}

#[test]
fn test_uuidv7_correct_format() {
    let uuid = umt_uuidv7();
    assert!(is_valid_uuidv7(&uuid), "UUID format incorrect: {}", uuid);
}

#[test]
fn test_uuidv7_unique() {
    let uuid1 = umt_uuidv7();
    let uuid2 = umt_uuidv7();
    assert_ne!(uuid1, uuid2);
}

#[test]
fn test_uuidv7_version() {
    let uuid = umt_uuidv7();
    let parts: Vec<&str> = uuid.split('-').collect();
    let version = parts[2].chars().next().unwrap();
    assert_eq!(version, '7');
}

#[test]
fn test_uuidv7_variant() {
    let uuid = umt_uuidv7();
    let parts: Vec<&str> = uuid.split('-').collect();
    let variant = parts[3].chars().next().unwrap();
    assert!(variant == '8' || variant == '9' || variant == 'a' || variant == 'b');
}

#[test]
fn test_uuidv7_non_colliding() {
    let mut uuids = HashSet::new();
    for _ in 0..1000 {
        let uuid = umt_uuidv7();
        assert!(!uuids.contains(&uuid));
        uuids.insert(uuid);
    }
}

#[test]
fn test_uuidv7_length() {
    let uuid = umt_uuidv7();
    assert_eq!(uuid.len(), 36);
}
