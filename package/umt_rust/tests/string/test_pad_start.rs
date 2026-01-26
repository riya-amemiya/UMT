use umt_rust::string::umt_pad_start;

// Basic functionality tests
#[test]
fn test_pad_start_of_string_with_specified_character() {
    assert_eq!(umt_pad_start("abc", 5, " "), "  abc");
    assert_eq!(umt_pad_start("abc", 5, "0"), "00abc");
}

#[test]
fn test_return_original_string_if_already_at_or_exceeding_target_length() {
    assert_eq!(umt_pad_start("abc", 2, " "), "abc"); // Shorter target
    assert_eq!(umt_pad_start("abc", 3, " "), "abc"); // Equal length
    assert_eq!(umt_pad_start("abc", 0, " "), "abc"); // Zero target
}

#[test]
fn test_handle_empty_string_input() {
    assert_eq!(umt_pad_start("", 3, "0"), "000");
    assert_eq!(umt_pad_start("", 4, "x"), "xxxx");
}

// Multi-character padding tests
#[test]
fn test_repeat_padding_string_to_reach_target_length() {
    assert_eq!(umt_pad_start("abc", 7, "xy"), "xyxyabc");
    assert_eq!(umt_pad_start("abc", 8, "123"), "12312abc");
}

#[test]
fn test_truncate_padding_string_if_needed() {
    assert_eq!(umt_pad_start("abc", 6, "12345"), "123abc");
    assert_eq!(umt_pad_start("abc", 5, "0000"), "00abc");
}

#[test]
fn test_work_with_single_character_padding_string() {
    assert_eq!(umt_pad_start("abc", 5, "x"), "xxabc");
}

// Empty padding string handling tests
#[test]
fn test_return_original_string_on_empty_padding_string() {
    assert_eq!(umt_pad_start("abc", 5, ""), "abc");
    assert_eq!(umt_pad_start("test", 10, ""), "test");
}

use umt_rust::string::*;

#[test]
fn test_pad_start_basic() {
    assert_eq!(umt_pad_start("123", 5, "0"), "00123");
}

#[test]
fn test_pad_start_empty_pad_string() {
    assert_eq!(umt_pad_start("hello", 10, ""), "hello");
}

#[test]
fn test_pad_start_no_padding_needed() {
    assert_eq!(umt_pad_start("hello", 3, "x"), "hello");
}

#[test]
fn test_pad_start_pattern() {
    assert_eq!(umt_pad_start("abc", 8, "def"), "defdeabc");
}

#[test]
fn test_pad_start_same_length() {
    assert_eq!(umt_pad_start("hello", 5, "x"), "hello");
}
