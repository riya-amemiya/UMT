use regex::Regex;
use umt_rust::string::umt_words;

#[test]
fn test_splits_camel_case() {
    assert_eq!(umt_words("helloWorld", None), vec!["hello", "World"]);
}

#[test]
fn test_splits_acronym_followed_by_word() {
    assert_eq!(
        umt_words("XMLHttpRequest", None),
        vec!["XML", "Http", "Request"]
    );
}

#[test]
fn test_splits_on_dashes_and_underscores() {
    assert_eq!(umt_words("foo-bar_baz", None), vec!["foo", "bar", "baz"]);
}

#[test]
fn test_splits_on_whitespace() {
    assert_eq!(umt_words("hello world", None), vec!["hello", "world"]);
}

#[test]
fn test_returns_empty_for_empty_string() {
    let empty: Vec<String> = Vec::new();
    assert_eq!(umt_words("", None), empty);
}

#[test]
fn test_uses_a_custom_pattern_when_provided() {
    let pattern = Regex::new(r"[a-z]\d").unwrap();
    assert_eq!(
        umt_words("a1 b2 c3", Some(&pattern)),
        vec!["a1", "b2", "c3"]
    );
}

#[test]
fn test_returns_empty_array_when_custom_pattern_matches_nothing() {
    let pattern = Regex::new(r"\d+").unwrap();
    let empty: Vec<String> = Vec::new();
    assert_eq!(umt_words("hello", Some(&pattern)), empty);
}
