use umt_rust::string::umt_sanitize_string;

#[test]
fn test_keeps_printable_ascii_characters() {
    assert_eq!(umt_sanitize_string("Hello, World!"), "Hello, World!");
}

#[test]
fn test_keeps_all_printable_ascii_range() {
    let printable = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    assert_eq!(umt_sanitize_string(printable), printable);
}

#[test]
fn test_keeps_newlines_carriage_returns_and_tabs() {
    assert_eq!(
        umt_sanitize_string("line1\nline2\r\n\tindented"),
        "line1\nline2\r\n\tindented"
    );
}

#[test]
fn test_removes_ansi_escape_sequences() {
    assert_eq!(
        umt_sanitize_string("\u{001B}[31mred\u{001B}[0m"),
        "[31mred[0m"
    );
}

#[test]
fn test_removes_null_bytes() {
    assert_eq!(umt_sanitize_string("a\u{0000}b"), "ab");
}

#[test]
fn test_removes_c0_control_characters_except_tab_lf_cr() {
    assert_eq!(umt_sanitize_string("a\u{0001}b\u{0002}c\u{001F}d"), "abcd");
}

#[test]
fn test_removes_the_del_character() {
    assert_eq!(umt_sanitize_string("a\u{007F}b"), "ab");
}

#[test]
fn test_removes_non_ascii_characters() {
    assert_eq!(umt_sanitize_string("héllo"), "hllo");
    assert_eq!(umt_sanitize_string("日本語"), "");
    assert_eq!(umt_sanitize_string("emoji\u{1F600}here"), "emojihere");
}

#[test]
fn test_handles_empty_string() {
    assert_eq!(umt_sanitize_string(""), "");
}
