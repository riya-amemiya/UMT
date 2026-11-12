use umt_rust::string::umt_wrap_text;

#[test]
fn test_wrap_text_doc_examples() {
    assert_eq!(
        umt_wrap_text("The quick brown fox", 10),
        "The quick\nbrown fox"
    );
    assert_eq!(umt_wrap_text("a extraordinary b", 5), "a\nextraordinary\nb");
}

#[test]
fn test_wrap_text_line_exactly_width() {
    assert_eq!(umt_wrap_text("ab cd", 5), "ab cd");
    assert_eq!(umt_wrap_text("ab cd", 4), "ab\ncd");
}

#[test]
fn test_wrap_text_collapses_separator_runs() {
    assert_eq!(umt_wrap_text("  a \t b\x0C\x0Bc  ", 10), "a b c");
}

#[test]
fn test_wrap_text_keeps_line_breaks() {
    assert_eq!(umt_wrap_text("aaa bbb\nccc", 3), "aaa\nbbb\nccc");
    assert_eq!(umt_wrap_text("a\n\nb", 5), "a\n\nb");
    assert_eq!(umt_wrap_text("ab\r\ncd", 10), "ab\ncd");
}

#[test]
fn test_wrap_text_zero_width() {
    assert_eq!(umt_wrap_text("a b", 0), "a\nb");
}

#[test]
fn test_wrap_text_empty_or_blank_input() {
    assert_eq!(umt_wrap_text("", 10), "");
    assert_eq!(umt_wrap_text("   ", 10), "");
}

#[test]
fn test_wrap_text_measures_characters() {
    assert_eq!(umt_wrap_text("\u{1F600} a", 3), "\u{1F600} a");
}
