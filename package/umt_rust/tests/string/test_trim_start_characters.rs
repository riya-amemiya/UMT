use umt_rust::string::umt_trim_start_characters;

#[test]
fn test_remove_specified_characters_from_the_start_of_string() {
    assert_eq!(umt_trim_start_characters("---Hello", "-"), "Hello");
    assert_eq!(umt_trim_start_characters("!!!world", "!"), "world");
}

#[test]
fn test_handle_multiple_different_characters_to_trim() {
    assert_eq!(umt_trim_start_characters("...123test", ".123"), "test");
    assert_eq!(umt_trim_start_characters("---...text", ".-"), "text");
}

#[test]
fn test_return_original_string_if_no_characters_match() {
    assert_eq!(umt_trim_start_characters("hello", "x"), "hello");
    assert_eq!(umt_trim_start_characters("test", "xyz"), "test");
}

#[test]
fn test_handle_empty_input_string() {
    assert_eq!(umt_trim_start_characters("", "x"), "");
    assert_eq!(umt_trim_start_characters("", ""), "");
}

#[test]
fn test_return_empty_string_if_all_characters_are_trimmed() {
    assert_eq!(umt_trim_start_characters("xxxxx", "x"), "");
    assert_eq!(umt_trim_start_characters(".....", "."), "");
}

#[test]
fn test_return_original_string_if_trim_characters_is_empty() {
    assert_eq!(umt_trim_start_characters("hello", ""), "hello");
    assert_eq!(umt_trim_start_characters("123test", ""), "123test");
}

#[test]
fn test_handle_non_ascii_characters() {
    assert_eq!(umt_trim_start_characters("。。。こんにちは", "。"), "こんにちは");
    assert_eq!(umt_trim_start_characters("！！Hello", "！"), "Hello");
}
