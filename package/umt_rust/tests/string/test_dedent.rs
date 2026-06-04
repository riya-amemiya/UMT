use umt_rust::string::umt_dedent;

#[test]
fn test_removes_minimum_common_indentation() {
    assert_eq!(
        umt_dedent("    line1\n      line2\n    line3"),
        "line1\n  line2\nline3"
    );
}

#[test]
fn test_ignores_blank_lines_when_computing_minimum() {
    assert_eq!(umt_dedent("    a\n\n    b"), "a\n\nb");
}

#[test]
fn test_returns_input_unchanged_when_no_indentation() {
    assert_eq!(umt_dedent("no indent"), "no indent");
}

#[test]
fn test_handles_all_blank_input() {
    assert_eq!(umt_dedent("\n  \n"), "\n  \n");
}
