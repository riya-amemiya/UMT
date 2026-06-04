use umt_rust::string::umt_to_full_width;

#[test]
fn test_converts_half_width_digits_to_full_width() {
    assert_eq!(umt_to_full_width("123"), "１２３");
}

#[test]
fn test_converts_half_width_uppercase_letters() {
    assert_eq!(umt_to_full_width("ABC"), "ＡＢＣ");
}

#[test]
fn test_converts_half_width_lowercase_letters() {
    assert_eq!(umt_to_full_width("abc"), "ａｂｃ");
}

#[test]
fn test_converts_mixed_alphanumeric_characters() {
    assert_eq!(umt_to_full_width("Abc123"), "Ａｂｃ１２３");
}

#[test]
fn test_leaves_non_alphanumeric_characters_unchanged() {
    assert_eq!(umt_to_full_width("a-b!"), "ａ-ｂ!");
}

#[test]
fn test_handles_empty_string() {
    assert_eq!(umt_to_full_width(""), "");
}
