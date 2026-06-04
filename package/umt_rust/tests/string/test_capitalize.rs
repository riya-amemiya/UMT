use umt_rust::string::umt_capitalize;

#[test]
fn test_uppercases_the_first_letter() {
    assert_eq!(umt_capitalize("hello"), "Hello");
}

#[test]
fn test_preserves_the_rest_of_the_string() {
    assert_eq!(umt_capitalize("hELLO"), "HELLO");
}

#[test]
fn test_handles_accented_first_letter() {
    assert_eq!(umt_capitalize("éclair"), "Éclair");
}

#[test]
fn test_returns_empty_for_empty_input() {
    assert_eq!(umt_capitalize(""), "");
}

#[test]
fn test_handles_surrogate_pair_first_character() {
    let input = "\u{1F600}abc";
    assert_eq!(umt_capitalize(input), input);
}
