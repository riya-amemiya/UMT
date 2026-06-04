use std::collections::HashSet;
use umt_rust::random::umt_random_uuid;

fn is_hex(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

fn matches_v4(uuid: &str) -> bool {
    let parts: Vec<&str> = uuid.split('-').collect();
    if parts.len() != 5 {
        return false;
    }
    let lengths = [8usize, 4, 4, 4, 12];
    for (part, len) in parts.iter().zip(lengths.iter()) {
        if part.len() != *len || !is_hex(part) {
            return false;
        }
    }
    // version nibble must be 4
    if !parts[2].starts_with('4') {
        return false;
    }
    // variant nibble must be one of 8, 9, a, b
    let variant = parts[3].chars().next().unwrap();
    matches!(variant, '8' | '9' | 'a' | 'b')
}

#[test]
fn test_returns_a_v4_formatted_uuid() {
    assert!(matches_v4(&umt_random_uuid()));
}

#[test]
fn test_returns_unique_values_across_calls() {
    let mut set = HashSet::new();
    for _ in 0..100 {
        set.insert(umt_random_uuid());
    }
    assert_eq!(set.len(), 100);
}
