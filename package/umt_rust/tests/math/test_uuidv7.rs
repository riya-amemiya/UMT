use std::collections::HashSet;
use umt_rust::math::umt_uuidv7;

#[test]
fn test_uuidv7_correct_format() {
    let uuid = umt_uuidv7();
    // Format: xxxxxxxx-xxxx-7xxx-Vxxx-xxxxxxxxxxxx
    let regex = regex::Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$").unwrap();
    assert!(regex.is_match(&uuid), "UUID format incorrect: {}", uuid);
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
