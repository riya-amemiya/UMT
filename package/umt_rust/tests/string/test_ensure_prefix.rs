use umt_rust::string::umt_ensure_prefix;

#[test]
fn test_prepends_the_prefix_when_missing() {
    assert_eq!(
        umt_ensure_prefix("example.com", "https://"),
        "https://example.com"
    );
}

#[test]
fn test_does_not_duplicate_an_existing_prefix() {
    assert_eq!(
        umt_ensure_prefix("https://example.com", "https://"),
        "https://example.com"
    );
}

#[test]
fn test_handles_empty_input_string() {
    assert_eq!(umt_ensure_prefix("", "/"), "/");
}

#[test]
fn test_returns_string_unchanged_for_empty_prefix() {
    assert_eq!(umt_ensure_prefix("abc", ""), "abc");
}
