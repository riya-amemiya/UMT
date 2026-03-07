use umt_rust::tool::umt_escape_regexp;

#[test]
fn test_escape_all_regex_special_characters() {
    let special_chars = ".*+?^${}()|[]\\";
    let expected = "\\.\\*\\+\\?\\^\\$\\{\\}\\(\\)\\|\\[\\]\\\\";
    assert_eq!(umt_escape_regexp(special_chars), expected);
}

#[test]
fn test_escape_each_character_correctly_individually() {
    assert_eq!(umt_escape_regexp("."), "\\.");
    assert_eq!(umt_escape_regexp("*"), "\\*");
    assert_eq!(umt_escape_regexp("+"), "\\+");
    assert_eq!(umt_escape_regexp("?"), "\\?");
    assert_eq!(umt_escape_regexp("^"), "\\^");
    assert_eq!(umt_escape_regexp("$"), "\\$");
    assert_eq!(umt_escape_regexp("{"), "\\{");
    assert_eq!(umt_escape_regexp("}"), "\\}");
    assert_eq!(umt_escape_regexp("("), "\\(");
    assert_eq!(umt_escape_regexp(")"), "\\)");
    assert_eq!(umt_escape_regexp("|"), "\\|");
    assert_eq!(umt_escape_regexp("["), "\\[");
    assert_eq!(umt_escape_regexp("]"), "\\]");
    assert_eq!(umt_escape_regexp("\\"), "\\\\");
}

#[test]
fn test_not_escape_alphanumeric_characters() {
    let string_ = "abcABC123";
    assert_eq!(umt_escape_regexp(string_), string_);
}

#[test]
fn test_handle_mixed_strings() {
    let string_ = "a.b+c";
    assert_eq!(umt_escape_regexp(string_), "a\\.b\\+c");
}

#[test]
fn test_handle_empty_string() {
    assert_eq!(umt_escape_regexp(""), "");
}
