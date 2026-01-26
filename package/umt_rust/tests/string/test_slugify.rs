use umt_rust::string::umt_slugify;

#[test]
fn test_convert_text_to_url_friendly_slug_according_to_jsdoc_examples() {
    assert_eq!(umt_slugify("Hello World!"), "hello-world");
    assert_eq!(umt_slugify("This is a Test"), "this-is-a-test");
    // Note: Rust implementation keeps unicode alphanumeric characters
    assert_eq!(umt_slugify("Japanese: こんにちは"), "japanese-こんにちは");
}

#[test]
fn test_handle_multiple_spaces() {
    assert_eq!(umt_slugify("Hello    World"), "hello-world");
}

#[test]
fn test_handle_leading_and_trailing_spaces() {
    assert_eq!(umt_slugify("  Hello World  "), "hello-world");
}

#[test]
fn test_remove_special_characters() {
    assert_eq!(umt_slugify("Special!@#$%Characters"), "special-characters");
}

#[test]
fn test_handle_underscores() {
    assert_eq!(umt_slugify("snake_case_text"), "snake-case-text");
}

#[test]
fn test_handle_unicode_characters() {
    assert_eq!(umt_slugify("café"), "cafe");
    assert_eq!(umt_slugify("naïve"), "naive");
}

#[test]
fn test_handle_numbers() {
    assert_eq!(umt_slugify("Test 123"), "test-123");
    assert_eq!(umt_slugify("Version 2.5"), "version-2-5");
}

#[test]
fn test_handle_empty_string() {
    assert_eq!(umt_slugify(""), "");
}

#[test]
fn test_handle_consecutive_hyphens() {
    assert_eq!(umt_slugify("Hello---World"), "hello-world");
}

#[test]
fn test_handle_mixed_case() {
    assert_eq!(umt_slugify("CamelCase"), "camelcase");
}

use umt_rust::string::*;

#[test]
fn test_slugify_accented() {
    assert_eq!(umt_slugify("cafe"), "cafe");
}

#[test]
fn test_slugify_basic() {
    assert_eq!(umt_slugify("Hello World!"), "hello-world");
}

#[test]
fn test_slugify_empty() {
    assert_eq!(umt_slugify(""), "");
}

#[test]
fn test_slugify_sentence() {
    assert_eq!(umt_slugify("This is a Test"), "this-is-a-test");
}

#[test]
fn test_slugify_special_chars() {
    assert_eq!(umt_slugify("Hello, World!"), "hello-world");
}
