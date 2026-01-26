use umt_rust::string::{umt_truncate, umt_truncate_default};

#[test]
fn test_truncate_string_according_to_jsdoc_examples() {
    assert_eq!(umt_truncate("Hello World", 5, "..."), "Hello...");
    assert_eq!(umt_truncate("Hello World", 5, "~"), "Hello~");
    assert_eq!(umt_truncate("Hello", 10, "..."), "Hello");
}

#[test]
fn test_not_truncate_if_string_is_shorter_than_or_equal_to_length() {
    assert_eq!(umt_truncate("Hi", 5, "..."), "Hi");
    assert_eq!(umt_truncate("Hello", 5, "..."), "Hello");
}

#[test]
fn test_handle_empty_suffix() {
    assert_eq!(umt_truncate("Hello World", 5, ""), "Hello");
}

#[test]
fn test_handle_zero_length() {
    assert_eq!(umt_truncate("Hello", 0, ""), "");
    assert_eq!(umt_truncate("Hello", 0, "..."), "...");
}

#[test]
fn test_handle_suffix_longer_than_target_length() {
    assert_eq!(umt_truncate("Hello World", 2, "..."), "He...");
    assert_eq!(umt_truncate("Hello World", 1, "..."), "H...");
}

#[test]
fn test_handle_empty_string() {
    assert_eq!(umt_truncate("", 5, "..."), "");
}

#[test]
fn test_truncate_default() {
    assert_eq!(umt_truncate_default("Hello World", 5), "Hello...");
}
