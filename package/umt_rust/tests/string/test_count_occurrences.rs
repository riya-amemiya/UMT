use umt_rust::string::umt_count_occurrences;

#[test]
fn test_counts_non_overlapping_occurrences() {
    assert_eq!(umt_count_occurrences("ababab", "ab"), 3);
}

#[test]
fn test_does_not_count_overlapping_occurrences() {
    assert_eq!(umt_count_occurrences("aaaa", "aa"), 2);
}

#[test]
fn test_counts_single_character_occurrences() {
    assert_eq!(umt_count_occurrences("banana", "a"), 3);
}

#[test]
fn test_returns_zero_when_substring_absent() {
    assert_eq!(umt_count_occurrences("hello", "z"), 0);
}

#[test]
fn test_returns_zero_for_empty_search_string() {
    assert_eq!(umt_count_occurrences("hello", ""), 0);
}

#[test]
fn test_returns_zero_for_empty_input_string() {
    assert_eq!(umt_count_occurrences("", "a"), 0);
}
