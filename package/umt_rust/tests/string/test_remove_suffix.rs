use umt_rust::string::umt_remove_suffix;

#[test]
fn test_removes_the_suffix_when_present() {
    assert_eq!(umt_remove_suffix("file.txt", ".txt"), "file");
}

#[test]
fn test_leaves_string_unchanged_when_suffix_absent() {
    assert_eq!(umt_remove_suffix("file", ".txt"), "file");
}

#[test]
fn test_removes_the_suffix_only_once() {
    assert_eq!(umt_remove_suffix("path//", "/"), "path/");
}

#[test]
fn test_returns_string_unchanged_for_empty_suffix() {
    assert_eq!(umt_remove_suffix("abc", ""), "abc");
}

#[test]
fn test_handles_empty_input_string() {
    assert_eq!(umt_remove_suffix("", "x"), "");
}
