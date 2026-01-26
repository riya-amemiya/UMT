use umt_rust::string::umt_to_half_width;

#[test]
fn test_convert_full_width_alphanumeric_characters_to_half_width() {
    assert_eq!(umt_to_half_width("０１２３４５６７８９"), "0123456789");
    assert_eq!(umt_to_half_width("ＡＢＣＤＥＦＧＨＩＪ"), "ABCDEFGHIJ");
    assert_eq!(umt_to_half_width("ａｂｃｄｅｆｇｈｉｊ"), "abcdefghij");
}

#[test]
fn test_correctly_convert_strings_containing_both_full_width_and_half_width_characters() {
    assert_eq!(umt_to_half_width("ＡＢＣabc１２３123"), "ABCabc123123");
}

#[test]
fn test_keep_non_target_characters_unchanged() {
    assert_eq!(umt_to_half_width("漢字カタカナ、。"), "漢字カタカナ、。");
}

use umt_rust::string::*;

#[test]
fn test_to_half_width_digits() {
    assert_eq!(umt_to_half_width("\u{FF11}\u{FF12}\u{FF13}"), "123");
}

#[test]
fn test_to_half_width_empty() {
    assert_eq!(umt_to_half_width(""), "");
}

#[test]
fn test_to_half_width_lowercase() {
    assert_eq!(umt_to_half_width("\u{FF41}\u{FF42}\u{FF43}"), "abc");
}

#[test]
fn test_to_half_width_mixed() {
    assert_eq!(umt_to_half_width("\u{FF21}1\u{FF42}2"), "A1b2");
}

#[test]
fn test_to_half_width_no_conversion() {
    assert_eq!(umt_to_half_width("ABC123"), "ABC123");
}

#[test]
fn test_to_half_width_uppercase() {
    assert_eq!(umt_to_half_width("\u{FF21}\u{FF22}\u{FF23}"), "ABC");
}
