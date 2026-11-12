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

#[test]
fn test_trims_a_character_set_from_both_ends() {
    assert_eq!(umt_trim_characters("abXba", "ab"), "X");
    assert_eq!(umt_trim_characters("aXbXa", "a"), "XbX");
    assert_eq!(umt_trim_characters("aaaa", "a"), "");
    assert_eq!(umt_trim_characters("\u{3042}a\u{3042}", "\u{3042}"), "a");
}
