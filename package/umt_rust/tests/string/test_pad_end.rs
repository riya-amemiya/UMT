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

#[test]
fn test_length_is_counted_in_characters() {
    assert_eq!(umt_pad_end("\u{3042}", 3, "x"), "\u{3042}xx");
    assert_eq!(
        umt_pad_end("a", 6, "\u{3042}\u{3044}"),
        "a\u{3042}\u{3044}\u{3042}\u{3044}\u{3042}"
    );
}

#[test]
fn test_string_already_at_target_length() {
    assert_eq!(umt_pad_end("abc", 3, "x"), "abc");
    assert_eq!(umt_pad_end("abc", 2, "x"), "abc");
}

#[test]
fn test_pad_string_is_cycled() {
    assert_eq!(umt_pad_end("", 3, "ab"), "aba");
}
