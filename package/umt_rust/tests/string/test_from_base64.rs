use umt_rust::string::{umt_from_base64, umt_to_base64};

#[test]
fn test_convert_base64_to_normal_string() {
    assert_eq!(umt_from_base64("dGVzdA==").unwrap(), "test");
    assert_eq!(umt_from_base64(&umt_to_base64("test")).unwrap(), "test");
}

#[test]
fn test_convert_empty_base64_string() {
    assert_eq!(umt_from_base64("").unwrap(), "");
}

#[test]
fn test_convert_base64_string_containing_special_characters() {
    assert_eq!(umt_from_base64("QCMl").unwrap(), "@#%");
    assert_eq!(umt_from_base64(&umt_to_base64("@#%")).unwrap(), "@#%");
}

#[test]
fn test_convert_base64_string_containing_japanese_characters() {
    assert_eq!(
        umt_from_base64("44GC44GE44GG44GI44GK").unwrap(),
        "あいうえお"
    );
    assert_eq!(
        umt_from_base64(&umt_to_base64("あいうえお")).unwrap(),
        "あいうえお"
    );
}

#[test]
fn test_convert_base64_string_containing_emojis() {
    assert_eq!(umt_from_base64(&umt_to_base64("🌊🌍🌎")).unwrap(), "🌊🌍🌎");
}

#[test]
fn test_handle_different_base64_padding_patterns() {
    assert_eq!(umt_from_base64("YQ==").unwrap(), "a"); // 2 padding chars
    assert_eq!(umt_from_base64("YWE=").unwrap(), "aa"); // 1 padding char
    assert_eq!(umt_from_base64("YWFh").unwrap(), "aaa"); // no padding
}

#[test]
fn test_throw_on_invalid_base64_string() {
    assert!(umt_from_base64("abc@!#").is_err());
    assert!(umt_from_base64("=abc").is_err());
}

use umt_rust::string::*;

#[test]
fn test_from_base64_basic() {
    assert_eq!(umt_from_base64("SGVsbG8gV29ybGQ=").unwrap(), "Hello World");
}

#[test]
fn test_from_base64_empty() {
    assert_eq!(umt_from_base64("").unwrap(), "");
}

#[test]
fn test_from_base64_invalid() {
    assert!(umt_from_base64("!!!invalid!!!").is_err());
}
