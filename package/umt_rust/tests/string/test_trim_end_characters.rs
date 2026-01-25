use umt_rust::string::umt_trim_end_characters;

#[test]
fn test_remove_specified_characters_from_the_end_of_string() {
    assert_eq!(umt_trim_end_characters("hellooo", "o"), "hell");
    assert_eq!(umt_trim_end_characters("banana!!!", "!"), "banana");
}

#[test]
fn test_handle_multiple_different_characters_to_trim() {
    assert_eq!(umt_trim_end_characters("hello123...", "123."), "hello");
    assert_eq!(umt_trim_end_characters("test---...", ".-"), "test");
}

#[test]
fn test_return_original_string_if_no_characters_match() {
    assert_eq!(umt_trim_end_characters("apple", "x"), "apple");
    assert_eq!(umt_trim_end_characters("test", "xyz"), "test");
}

#[test]
fn test_handle_empty_input_string() {
    assert_eq!(umt_trim_end_characters("", "x"), "");
    assert_eq!(umt_trim_end_characters("", ""), "");
}

#[test]
fn test_return_empty_string_if_all_characters_are_trimmed() {
    assert_eq!(umt_trim_end_characters("xxxxx", "x"), "");
    assert_eq!(umt_trim_end_characters("....", "."), "");
}

#[test]
fn test_return_original_string_if_trim_characters_is_empty() {
    assert_eq!(umt_trim_end_characters("hello", ""), "hello");
    assert_eq!(umt_trim_end_characters("test123", ""), "test123");
}

#[test]
fn test_handle_non_ascii_characters() {
    assert_eq!(umt_trim_end_characters("こんにちは。。。", "。"), "こんにちは");
    assert_eq!(umt_trim_end_characters("Hello！！", "！"), "Hello");
}
