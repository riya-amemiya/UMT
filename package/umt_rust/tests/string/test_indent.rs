use umt_rust::string::{umt_dedent, umt_indent, umt_indent_default};

#[test]
fn test_indent_doc_examples() {
    assert_eq!(umt_indent("line1\nline2", 2, " "), "  line1\n  line2");
    assert_eq!(umt_indent("a\n\nb", 1, "\t"), "\ta\n\n\tb");
}

#[test]
fn test_indent_whitespace_only_line_is_prefixed() {
    assert_eq!(umt_indent("  \nx", 2, " "), "    \n  x");
}

#[test]
fn test_indent_multi_character_unit() {
    assert_eq!(umt_indent("x", 2, "ab"), "ababx");
}

#[test]
fn test_indent_zero_count() {
    assert_eq!(umt_indent("a\nb", 0, " "), "a\nb");
}

#[test]
fn test_indent_empty_input() {
    assert_eq!(umt_indent("", 4, " "), "");
    assert_eq!(umt_indent("\n", 4, " "), "\n");
}

#[test]
fn test_indent_reversed_by_dedent() {
    let original = "a\n  b\n\nc";
    assert_eq!(umt_dedent(&umt_indent(original, 4, " ")), original);
}

#[test]
fn test_indent_default() {
    assert_eq!(umt_indent_default("line1\nline2", 2), "  line1\n  line2");
}
