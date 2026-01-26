use umt_rust::string::umt_delete_spaces;

#[test]
fn test_remove_spaces_from_string() {
    assert_eq!(umt_delete_spaces("Hello World"), "HelloWorld");
    assert_eq!(umt_delete_spaces("   Hello   World   "), "HelloWorld");
}

#[test]
fn test_remove_tabs_and_other_whitespace_characters() {
    assert_eq!(umt_delete_spaces("Hello\tWorld"), "HelloWorld");
    assert_eq!(umt_delete_spaces("Hello\nWorld\r\n"), "HelloWorld");
    // Note: Em space U+2003 should be removed as well
    assert_eq!(umt_delete_spaces("Hello\u{2003}World"), "HelloWorld");
}

#[test]
fn test_handle_empty_string() {
    assert_eq!(umt_delete_spaces(""), "");
    assert_eq!(umt_delete_spaces("   "), "");
}

#[test]
fn test_handle_string_with_multibyte_characters() {
    assert_eq!(umt_delete_spaces("こんにちは 世界"), "こんにちは世界");
    // Full-width space U+3000
    assert_eq!(umt_delete_spaces("Hello\u{3000}World"), "HelloWorld");
}

use umt_rust::string::*;

#[test]
fn test_delete_spaces_basic() {
    assert_eq!(umt_delete_spaces("Hello World"), "HelloWorld");
}

#[test]
fn test_delete_spaces_empty() {
    assert_eq!(umt_delete_spaces(""), "");
}

#[test]
fn test_delete_spaces_no_spaces() {
    assert_eq!(umt_delete_spaces("HelloWorld"), "HelloWorld");
}

#[test]
fn test_delete_spaces_tabs() {
    assert_eq!(umt_delete_spaces("  tab\t space "), "tabspace");
}
