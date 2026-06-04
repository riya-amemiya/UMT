use umt_rust::string::umt_remove_prefix;

#[test]
fn test_removes_the_prefix_when_present() {
    assert_eq!(
        umt_remove_prefix("https://example.com", "https://"),
        "example.com"
    );
}

#[test]
fn test_leaves_string_unchanged_when_prefix_absent() {
    assert_eq!(umt_remove_prefix("example.com", "https://"), "example.com");
}

#[test]
fn test_removes_the_prefix_only_once() {
    assert_eq!(umt_remove_prefix("//path", "/"), "/path");
}

#[test]
fn test_returns_string_unchanged_for_empty_prefix() {
    assert_eq!(umt_remove_prefix("abc", ""), "abc");
}

#[test]
fn test_handles_empty_input_string() {
    assert_eq!(umt_remove_prefix("", "x"), "");
}
