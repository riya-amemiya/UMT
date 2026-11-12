use umt_rust::string::{umt_damerau_levenshtein_distance, umt_levenshtein_distance};

#[test]
fn test_damerau_levenshtein_distance_doc_examples() {
    assert_eq!(umt_damerau_levenshtein_distance("ca", "ac"), 1);
    assert_eq!(umt_damerau_levenshtein_distance("kitten", "sitting"), 3);
    assert_eq!(umt_damerau_levenshtein_distance("ca", "abc"), 3);
}

#[test]
fn test_damerau_levenshtein_distance_identical() {
    assert_eq!(umt_damerau_levenshtein_distance("hello", "hello"), 0);
    assert_eq!(umt_damerau_levenshtein_distance("", ""), 0);
}

#[test]
fn test_damerau_levenshtein_distance_empty() {
    assert_eq!(umt_damerau_levenshtein_distance("", "abc"), 3);
    assert_eq!(umt_damerau_levenshtein_distance("abc", ""), 3);
}

#[test]
fn test_damerau_levenshtein_distance_adjacent_transposition() {
    assert_eq!(umt_damerau_levenshtein_distance("ab", "ba"), 1);
    assert_eq!(umt_levenshtein_distance("ab", "ba"), 2);
    assert_eq!(umt_damerau_levenshtein_distance("abcd", "acbd"), 1);
    assert_eq!(umt_damerau_levenshtein_distance("abcdef", "badcfe"), 3);
}

#[test]
fn test_damerau_levenshtein_distance_single_edits() {
    assert_eq!(umt_damerau_levenshtein_distance("a", "b"), 1);
    assert_eq!(umt_damerau_levenshtein_distance("cat", "cats"), 1);
    assert_eq!(umt_damerau_levenshtein_distance("cats", "cat"), 1);
}

#[test]
fn test_damerau_levenshtein_distance_transposition_not_cheaper() {
    assert_eq!(umt_damerau_levenshtein_distance("aax", "aay"), 1);
}

#[test]
fn test_damerau_levenshtein_distance_symmetric() {
    assert_eq!(umt_damerau_levenshtein_distance("abc", "ca"), 3);
    assert_eq!(umt_damerau_levenshtein_distance("sitting", "kitten"), 3);
}

#[test]
fn test_damerau_levenshtein_distance_long_inputs_rotate_rows() {
    assert_eq!(
        umt_damerau_levenshtein_distance("abcdefghij", "bacdefghji"),
        2
    );
}

#[test]
fn test_damerau_levenshtein_distance_compares_characters() {
    assert_eq!(
        umt_damerau_levenshtein_distance("\u{1F600}x", "x\u{1F600}"),
        1
    );
}
