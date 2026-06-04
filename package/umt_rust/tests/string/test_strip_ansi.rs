use umt_rust::string::umt_strip_ansi;

#[test]
fn test_removes_sgr_color_codes() {
    assert_eq!(umt_strip_ansi("\u{001B}[31mred\u{001B}[0m"), "red");
}

#[test]
fn test_removes_multiple_sequences() {
    assert_eq!(
        umt_strip_ansi("\u{001B}[1m\u{001B}[32mbold green\u{001B}[0m"),
        "bold green"
    );
}

#[test]
fn test_removes_cursor_movement_sequences() {
    assert_eq!(umt_strip_ansi("a\u{001B}[2Ab"), "ab");
}

#[test]
fn test_leaves_plain_text_unchanged() {
    assert_eq!(umt_strip_ansi("plain text"), "plain text");
}

#[test]
fn test_handles_empty_string() {
    assert_eq!(umt_strip_ansi(""), "");
}
