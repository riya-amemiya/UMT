//! Tests for string length validation measured in UTF-16 code units

use umt_rust::validate::string::umt_length;

fn repeated_char(code: u32, count: usize) -> String {
    char::from_u32(code).unwrap().to_string().repeat(count)
}

#[test]
fn test_length_counts_basic_multilingual_plane_characters_once() {
    let validator = umt_length(3, None);
    assert!((validator.validate)(&repeated_char(0x3042, 3)));
    assert!(!(validator.validate)(&repeated_char(0x3042, 1)));
    assert!(!(validator.validate)(&repeated_char(0x3042, 9)));
}

#[test]
fn test_length_counts_astral_characters_twice() {
    let validator = umt_length(2, None);
    assert!((validator.validate)(&repeated_char(0x1F600, 1)));
    assert!(!(validator.validate)(&repeated_char(0x1F600, 2)));
}

#[test]
fn test_length_counts_ascii_characters() {
    let validator = umt_length(3, None);
    assert!((validator.validate)(&"abc".to_string()));
    assert!(!(validator.validate)(&"ab".to_string()));
    assert!((umt_length(0, None).validate)(&String::new()));
}
