use umt_rust::string::umt_capitalize_word;

#[test]
fn test_uppercases_first_lowercases_rest() {
    assert_eq!(umt_capitalize_word("hello"), "Hello");
}

#[test]
fn test_normalizes_mixed_case_input() {
    assert_eq!(umt_capitalize_word("hELLO"), "Hello");
}

#[test]
fn test_returns_empty_for_empty_input() {
    assert_eq!(umt_capitalize_word(""), "");
}

#[test]
fn test_preserves_single_character_input() {
    assert_eq!(umt_capitalize_word("a"), "A");
}
