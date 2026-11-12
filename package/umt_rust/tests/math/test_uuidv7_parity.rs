use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use umt_rust::math::umt_uuidv7;

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[test]
fn test_uuidv7_timestamp_prefix_matches_current_time() {
    let before = now_ms();
    let id = umt_uuidv7();
    let after = now_ms();
    let timestamp = u64::from_str_radix(&format!("{}{}", &id[0..8], &id[9..13]), 16).unwrap();
    assert!(timestamp >= before && timestamp <= after);
}

#[test]
fn test_uuidv7_version_and_variant_bits() {
    for _ in 0..200 {
        let id = umt_uuidv7();
        assert_eq!(&id[14..15], "7");
        assert!(matches!(&id[19..20], "8" | "9" | "a" | "b"));
    }
}

#[test]
fn test_uuidv7_random_nibbles_cover_every_value() {
    let ids: Vec<String> = (0..2000).map(|_| umt_uuidv7()).collect();
    let version_neighbors: HashSet<&str> = ids.iter().map(|id| &id[15..16]).collect();
    let last_nibbles: HashSet<&str> = ids.iter().map(|id| &id[35..36]).collect();
    assert_eq!(version_neighbors.len(), 16);
    assert_eq!(last_nibbles.len(), 16);
}
