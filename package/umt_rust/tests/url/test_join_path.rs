use umt_rust::url::umt_join_path;

#[test]
fn test_joins_simple_segments() {
    assert_eq!(umt_join_path(&["a", "b", "c"]), "a/b/c");
}

#[test]
fn test_normalizes_slashes_between_segments() {
    assert_eq!(umt_join_path(&["a/", "/b/", "/c"]), "a/b/c");
}

#[test]
fn test_preserves_leading_slash_of_first_segment() {
    assert_eq!(umt_join_path(&["/a", "b", "c"]), "/a/b/c");
}

#[test]
fn test_preserves_trailing_slash_of_last_segment() {
    assert_eq!(umt_join_path(&["a", "b", "c/"]), "a/b/c/");
}

#[test]
fn test_handles_url_like_base_segments() {
    assert_eq!(
        umt_join_path(&["https://example.com/", "/api/", "/users"]),
        "https://example.com/api/users"
    );
}

#[test]
fn test_handles_single_segment() {
    assert_eq!(umt_join_path(&["path"]), "path");
}

#[test]
fn test_returns_empty_string_for_no_segments() {
    assert_eq!(umt_join_path(&[]), "");
}

#[test]
fn test_handles_multiple_slashes() {
    assert_eq!(umt_join_path(&["a///", "///b"]), "a/b");
}

#[test]
fn test_handles_empty_segments() {
    assert_eq!(umt_join_path(&["a", "", "b"]), "a/b");
}
