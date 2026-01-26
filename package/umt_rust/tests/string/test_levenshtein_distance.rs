use umt_rust::string::umt_levenshtein_distance;

#[test]
fn test_return_0_for_identical_strings() {
    assert_eq!(umt_levenshtein_distance("hello", "hello"), 0);
    assert_eq!(umt_levenshtein_distance("", ""), 0);
}

#[test]
fn test_handle_empty_strings() {
    assert_eq!(umt_levenshtein_distance("", "hello"), 5);
    assert_eq!(umt_levenshtein_distance("hello", ""), 5);
}

#[test]
fn test_calculate_distance_for_single_character_differences() {
    assert_eq!(umt_levenshtein_distance("cat", "bat"), 1); // substitution
    assert_eq!(umt_levenshtein_distance("cat", "cats"), 1); // insertion
    assert_eq!(umt_levenshtein_distance("cats", "cat"), 1); // deletion
}

#[test]
fn test_calculate_distance_for_multiple_differences() {
    assert_eq!(umt_levenshtein_distance("kitten", "sitting"), 3);
    assert_eq!(umt_levenshtein_distance("saturday", "sunday"), 3);
}

#[test]
fn test_handle_completely_different_strings() {
    assert_eq!(umt_levenshtein_distance("abc", "xyz"), 3);
    assert_eq!(umt_levenshtein_distance("hello", "world"), 4);
}

#[test]
fn test_be_case_sensitive() {
    assert_eq!(umt_levenshtein_distance("Hello", "hello"), 1);
    assert_eq!(umt_levenshtein_distance("ABC", "abc"), 3);
}

#[test]
fn test_handle_unicode_characters() {
    assert_eq!(umt_levenshtein_distance("café", "cafe"), 1);
    assert_eq!(umt_levenshtein_distance("😀", "😁"), 1);
    assert_eq!(umt_levenshtein_distance("こんにちは", "こんばんは"), 2);
}

use umt_rust::string::*;

#[test]
fn test_levenshtein_distance_basic() {
    assert_eq!(umt_levenshtein_distance("kitten", "sitting"), 3);
}

#[test]
fn test_levenshtein_distance_empty() {
    assert_eq!(umt_levenshtein_distance("", "hello"), 5);
    assert_eq!(umt_levenshtein_distance("hello", ""), 5);
    assert_eq!(umt_levenshtein_distance("", ""), 0);
}

#[test]
fn test_levenshtein_distance_same() {
    assert_eq!(umt_levenshtein_distance("hello", "hello"), 0);
}

#[test]
fn test_levenshtein_distance_single_char() {
    assert_eq!(umt_levenshtein_distance("a", "b"), 1);
    assert_eq!(umt_levenshtein_distance("a", "a"), 0);
}
