use regex::Regex;
use umt_rust::string::{umt_random_string, umt_random_string_default};

#[test]
fn test_generate_random_string_with_default_characters_and_length() {
    let str_result = umt_random_string_default(8);
    assert_eq!(str_result.len(), 8);
    let pattern = Regex::new(r"^[0-9A-Za-z]{8}$").unwrap();
    assert!(pattern.is_match(&str_result));
}

#[test]
fn test_generate_random_string_using_custom_character_set() {
    let chars = "abc123";
    let str_result = umt_random_string(10, Some(chars));
    assert_eq!(str_result.len(), 10);
    let pattern = Regex::new(r"^[abc123]{10}$").unwrap();
    assert!(pattern.is_match(&str_result));
}

#[test]
fn test_generate_random_string_with_specified_length() {
    let size = 20;
    let str_result = umt_random_string(size, None);
    assert_eq!(str_result.len(), size);
}

#[test]
fn test_generate_random_string_with_zero_size() {
    let str_result = umt_random_string(0, None);
    assert!(str_result.is_empty());
}

#[test]
fn test_generate_random_string_custom_chars_all_included() {
    let chars = "ABC";
    let str_result = umt_random_string(100, Some(chars));
    // With 100 characters, it's very likely all chars appear
    assert!(str_result.chars().all(|c| chars.contains(c)));
}

use umt_rust::string::*;

#[test]
fn test_random_string_custom_chars() {
    let s = umt_random_string(10, Some("abc"));
    assert_eq!(s.len(), 10);
    assert!(s.chars().all(|c| "abc".contains(c)));
}

#[test]
fn test_random_string_default() {
    let s = umt_random_string_default(16);
    assert_eq!(s.len(), 16);
}

#[test]
fn test_random_string_empty_size() {
    let s = umt_random_string(0, None);
    assert_eq!(s.len(), 0);
}

#[test]
fn test_random_string_length() {
    let s = umt_random_string(8, None);
    assert_eq!(s.len(), 8);
}
