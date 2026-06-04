use umt_rust::string::umt_word_count;

#[test]
fn test_counts_space_separated_words() {
    assert_eq!(umt_word_count("hello world"), 2);
}

#[test]
fn test_counts_camel_case_boundaries() {
    assert_eq!(umt_word_count("camelCaseSplit"), 3);
}

#[test]
fn test_returns_zero_for_empty_string() {
    assert_eq!(umt_word_count(""), 0);
}

#[test]
fn test_counts_words_across_punctuation() {
    assert_eq!(umt_word_count("foo, bar! baz?"), 3);
}
