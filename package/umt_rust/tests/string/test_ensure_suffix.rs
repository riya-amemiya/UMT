use umt_rust::string::umt_ensure_suffix;

#[test]
fn test_appends_the_suffix_when_missing() {
    assert_eq!(umt_ensure_suffix("file", ".txt"), "file.txt");
}

#[test]
fn test_does_not_duplicate_an_existing_suffix() {
    assert_eq!(umt_ensure_suffix("file.txt", ".txt"), "file.txt");
}

#[test]
fn test_handles_empty_input_string() {
    assert_eq!(umt_ensure_suffix("", "/"), "/");
}

#[test]
fn test_returns_string_unchanged_for_empty_suffix() {
    assert_eq!(umt_ensure_suffix("abc", ""), "abc");
}
