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
