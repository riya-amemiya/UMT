use umt_rust::string::umt_normalize_whitespace;

#[test]
fn test_collapses_consecutive_spaces_into_one() {
    assert_eq!(umt_normalize_whitespace("hello   world"), "hello world");
}

#[test]
fn test_collapses_mixed_whitespace_into_single_spaces() {
    assert_eq!(umt_normalize_whitespace("hello \t\n world"), "hello world");
}

#[test]
fn test_trims_leading_and_trailing_whitespace() {
    assert_eq!(umt_normalize_whitespace("  hello world  "), "hello world");
}

#[test]
fn test_handles_string_of_only_whitespace() {
    assert_eq!(umt_normalize_whitespace("  \t\n "), "");
}

#[test]
fn test_leaves_single_spaced_string_unchanged() {
    assert_eq!(umt_normalize_whitespace("a b c"), "a b c");
}

#[test]
fn test_handles_empty_string() {
    assert_eq!(umt_normalize_whitespace(""), "");
}

#[test]
fn test_collapses_docstring_example() {
    assert_eq!(
        umt_normalize_whitespace("  hello   world \t\n foo "),
        "hello world foo"
    );
}

#[test]
fn test_collapses_carriage_return_whitespace() {
    assert_eq!(umt_normalize_whitespace("hello \r\n world"), "hello world");
}

#[test]
fn test_collapses_consecutive_newlines() {
    assert_eq!(umt_normalize_whitespace("a\n\n\nb"), "a b");
}

#[test]
fn test_keeps_text_around_whitespace_runs() {
    assert_eq!(
        umt_normalize_whitespace("keep \t  this \n text"),
        "keep this text"
    );
}
