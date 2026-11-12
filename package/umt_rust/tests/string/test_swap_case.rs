use umt_rust::string::umt_swap_case;

#[test]
fn test_swaps_the_case_of_each_letter() {
    assert_eq!(umt_swap_case("Hello World"), "hELLO wORLD");
}

#[test]
fn test_turns_all_uppercase_into_lowercase() {
    assert_eq!(umt_swap_case("ABC"), "abc");
}

#[test]
fn test_turns_all_lowercase_into_uppercase() {
    assert_eq!(umt_swap_case("abc"), "ABC");
}

#[test]
fn test_leaves_characters_without_case_unchanged() {
    assert_eq!(umt_swap_case("abc123-日本"), "ABC123-日本");
}

#[test]
fn test_handles_empty_string() {
    assert_eq!(umt_swap_case(""), "");
}

#[test]
fn test_non_letters_with_case_mappings_are_unchanged() {
    assert_eq!(umt_swap_case("\u{2177}"), "\u{2177}");
    assert_eq!(umt_swap_case("\u{2167}"), "\u{2167}");
    assert_eq!(umt_swap_case("\u{24d0}"), "\u{24d0}");
}

#[test]
fn test_multi_character_case_mappings() {
    assert_eq!(umt_swap_case("\u{df}"), "SS");
    assert_eq!(umt_swap_case("\u{fb01}"), "FI");
    assert_eq!(umt_swap_case("\u{130}"), "i\u{307}");
}

#[test]
fn test_titlecase_and_caseless_letters() {
    assert_eq!(umt_swap_case("\u{1c5}"), "\u{1c6}");
    assert_eq!(umt_swap_case("\u{1c6}"), "\u{1c4}");
    assert_eq!(umt_swap_case("\u{2b0}"), "\u{2b0}");
    assert_eq!(umt_swap_case("\u{3c3}\u{3c2}"), "\u{3a3}\u{3a3}");
    assert_eq!(umt_swap_case("aB1"), "Ab1");
}
