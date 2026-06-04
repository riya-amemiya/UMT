use umt_rust::object::umt_path_segments;

#[test]
fn test_splits_a_dotted_string_into_segments() {
    assert_eq!(umt_path_segments("a.b.c"), vec!["a", "b", "c"]);
}

#[test]
fn test_returns_single_element_for_non_dotted_string() {
    assert_eq!(umt_path_segments("a"), vec!["a"]);
}

#[test]
fn test_splits_empty_string_into_single_empty_segment() {
    assert_eq!(umt_path_segments(""), vec![""]);
}
