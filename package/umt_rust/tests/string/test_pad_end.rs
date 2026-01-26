use umt_rust::string::umt_pad_end;

#[test]
fn test_add_padding_to_end_of_string() {
    assert_eq!(umt_pad_end("abc", 5, " "), "abc  ");
    assert_eq!(umt_pad_end("hello", 10, "!"), "hello!!!!!");
}

#[test]
fn test_not_modify_if_original_string_is_already_at_or_longer_than_target_length() {
    assert_eq!(umt_pad_end("abc", 3, " "), "abc");
    assert_eq!(umt_pad_end("longstring", 5, "!"), "longstring");
}

#[test]
fn test_handle_padding_string_with_multiple_characters() {
    assert_eq!(umt_pad_end("abc", 10, "de"), "abcdededed");
}

#[test]
fn test_return_original_string_if_target_length_is_shorter_than_original_length() {
    assert_eq!(umt_pad_end("abc", 2, " "), "abc");
}

#[test]
fn test_return_original_string_if_padding_string_is_empty() {
    assert_eq!(umt_pad_end("abc", 5, ""), "abc");
}

use umt_rust::string::*;

#[test]
fn test_pad_end_basic() {
    assert_eq!(umt_pad_end("123", 5, "0"), "12300");
}

#[test]
fn test_pad_end_empty_pad_string() {
    assert_eq!(umt_pad_end("hello", 10, ""), "hello");
}

#[test]
fn test_pad_end_no_padding_needed() {
    assert_eq!(umt_pad_end("hello", 3, "x"), "hello");
}

#[test]
fn test_pad_end_pattern() {
    assert_eq!(umt_pad_end("abc", 8, "def"), "abcdefde");
}

#[test]
fn test_pad_end_same_length() {
    assert_eq!(umt_pad_end("hello", 5, "x"), "hello");
}
