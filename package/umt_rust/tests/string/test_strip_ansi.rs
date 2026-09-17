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

#[test]
fn test_keeps_text_around_sequences() {
    assert_eq!(
        umt_strip_ansi("hello \u{001B}[31mred\u{001B}[0m world"),
        "hello red world"
    );
}

#[test]
fn test_removes_osc_bel_sequences() {
    assert_eq!(umt_strip_ansi("\u{001B}]0;title\u{0007}hello"), "hello");
}

#[test]
fn test_removes_osc_st_sequences() {
    assert_eq!(umt_strip_ansi("\u{001B}]0;title\u{001B}\\hello"), "hello");
}

#[test]
fn test_removes_c1_csi_sequences() {
    assert_eq!(umt_strip_ansi("\u{009B}31mred"), "red");
}

#[test]
fn test_leaves_incomplete_csi_sequences() {
    assert_eq!(umt_strip_ansi("\u{001B}[31"), "\u{001B}[31");
}
