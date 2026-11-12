use umt_rust::string::{umt_truncate_middle, umt_truncate_middle_default};

#[test]
fn test_truncate_middle_doc_examples() {
    assert_eq!(umt_truncate_middle("Hello World", 8, "..."), "Hel...ld");
    assert_eq!(umt_truncate_middle("Hello World", 7, "~"), "Hel~rld");
    assert_eq!(umt_truncate_middle("Hello", 10, "..."), "Hello");
    assert_eq!(umt_truncate_middle("Hello World", 2, "..."), "..");
}

#[test]
fn test_truncate_middle_no_truncation_when_fits() {
    assert_eq!(umt_truncate_middle("Hello World", 11, "..."), "Hello World");
    assert_eq!(umt_truncate_middle("", 0, "..."), "");
}

#[test]
fn test_truncate_middle_extra_character_goes_to_head() {
    assert_eq!(umt_truncate_middle("Hello World", 10, "..."), "Hell...rld");
    assert_eq!(umt_truncate_middle("Hello World", 4, "..."), "H...");
}

#[test]
fn test_truncate_middle_never_exceeds_length() {
    let source = "abcdefghijklmnopqrstuvwxyz";
    for length in 0..=source.len() {
        assert!(umt_truncate_middle(source, length, "...").chars().count() <= length);
    }
}

#[test]
fn test_truncate_middle_separator_cut_when_length_too_small() {
    assert_eq!(umt_truncate_middle("Hello World", 3, "..."), "...");
    assert_eq!(umt_truncate_middle("Hello World", 1, "..."), ".");
    assert_eq!(umt_truncate_middle("Hello World", 0, "..."), "");
}

#[test]
fn test_truncate_middle_empty_separator() {
    assert_eq!(umt_truncate_middle("abcdef", 4, ""), "abef");
    assert_eq!(umt_truncate_middle("abcdef", 0, ""), "");
}

#[test]
fn test_truncate_middle_counts_characters() {
    assert_eq!(umt_truncate_middle("a\u{1F600}bc", 3, ""), "a\u{1F600}c");
}

#[test]
fn test_truncate_middle_default() {
    assert_eq!(umt_truncate_middle_default("Hello World", 8), "Hel...ld");
    assert_eq!(umt_truncate_middle_default("Hello", 10), "Hello");
}
