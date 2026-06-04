use umt_rust::string::umt_uncapitalize;

#[test]
fn test_lowercases_the_first_letter() {
    assert_eq!(umt_uncapitalize("Hello"), "hello");
}

#[test]
fn test_preserves_the_rest_of_the_string() {
    assert_eq!(umt_uncapitalize("ÉCLAIR"), "éCLAIR");
}

#[test]
fn test_returns_empty_for_empty_input() {
    assert_eq!(umt_uncapitalize(""), "");
}

#[test]
fn test_handles_surrogate_pair_first_character() {
    let input = "\u{1F600}ABC";
    assert_eq!(umt_uncapitalize(input), input);
}
