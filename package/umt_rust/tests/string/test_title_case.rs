use umt_rust::string::umt_title_case;

#[test]
fn test_converts_plain_sentence() {
    assert_eq!(umt_title_case("hello world"), "Hello World");
}

#[test]
fn test_converts_mixed_separators() {
    assert_eq!(umt_title_case("a-quick brown_fox"), "A Quick Brown Fox");
}

#[test]
fn test_converts_camel_case() {
    assert_eq!(umt_title_case("helloWorld"), "Hello World");
}

#[test]
fn test_returns_empty_for_empty_string() {
    assert_eq!(umt_title_case(""), "");
}
