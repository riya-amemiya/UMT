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
