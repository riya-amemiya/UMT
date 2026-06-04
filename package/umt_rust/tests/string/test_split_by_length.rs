use umt_rust::string::umt_split_by_length;

#[test]
fn test_splits_into_equal_chunks() {
    assert_eq!(umt_split_by_length("abcdef", 2), vec!["ab", "cd", "ef"]);
}

#[test]
fn test_keeps_the_remainder_in_the_last_chunk() {
    assert_eq!(umt_split_by_length("abcdefg", 3), vec!["abc", "def", "g"]);
}

#[test]
fn test_returns_single_chunk_when_length_exceeds_string() {
    assert_eq!(umt_split_by_length("abc", 10), vec!["abc"]);
}

#[test]
fn test_is_surrogate_pair_safe() {
    assert_eq!(umt_split_by_length("😀😁😂😃", 2), vec!["😀😁", "😂😃"]);
}

#[test]
fn test_returns_empty_array_for_empty_string() {
    let empty: Vec<String> = Vec::new();
    assert_eq!(umt_split_by_length("", 3), empty);
}
