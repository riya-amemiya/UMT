use umt_rust::string::umt_trim_characters;

#[test]
fn test_remove_specified_characters_from_both_ends_of_string() {
    assert_eq!(umt_trim_characters("---Hello World---", "-"), "Hello World");
}

#[test]
fn test_not_remove_specified_characters_from_middle_of_string() {
    assert_eq!(umt_trim_characters("---Hello-World---", "-"), "Hello-World");
}

#[test]
fn test_return_empty_string_when_input_is_empty() {
    assert_eq!(umt_trim_characters("", "-"), "");
}

#[test]
fn test_return_original_string_when_no_characters_are_specified_to_remove() {
    assert_eq!(umt_trim_characters("Hello World", ""), "Hello World");
}

use umt_rust::string::*;

#[test]
fn test_trim_characters_both() {
    assert_eq!(umt_trim_characters("!!!hello!!!", "!"), "hello");
}

#[test]
fn test_trim_characters_dashes() {
    assert_eq!(umt_trim_characters("---123---", "-"), "123");
}

#[test]
fn test_trim_characters_empty() {
    assert_eq!(umt_trim_characters("", "!"), "");
}

#[test]
fn test_trim_characters_no_match() {
    assert_eq!(umt_trim_characters("abc123", "xyz"), "abc123");
}
