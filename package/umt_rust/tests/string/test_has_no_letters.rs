use umt_rust::string::umt_has_no_letters;

#[test]
fn test_return_true_for_strings_without_any_letters() {
    assert!(umt_has_no_letters("")); // Empty string
    assert!(umt_has_no_letters("12345")); // Numbers only
    assert!(umt_has_no_letters("!@#$%")); // Special characters
    assert!(umt_has_no_letters("     ")); // Whitespace only
}

#[test]
fn test_return_true_for_strings_with_only_emojis_and_symbols() {
    assert!(umt_has_no_letters("🎉")); // Single emoji
    assert!(umt_has_no_letters("😀😃😄")); // Multiple emojis
    assert!(umt_has_no_letters("❥･•❄")); // Decorative symbols
    assert!(umt_has_no_letters("？？？？？")); // Full-width symbols
    assert!(umt_has_no_letters(
        "
      🎈
      🎊
      🎉
    "
    )); // Multiline emojis
}

#[test]
fn test_return_false_for_strings_containing_letters() {
    assert!(!umt_has_no_letters("hello")); // English text
    assert!(!umt_has_no_letters("test 123")); // Mixed with numbers
    assert!(!umt_has_no_letters("こんにちは")); // Japanese text
    assert!(!umt_has_no_letters("Café")); // Accented characters
}

#[test]
fn test_return_false_for_strings_with_mixed_content() {
    assert!(!umt_has_no_letters("hello 👋")); // Text with emoji
    assert!(!umt_has_no_letters("🎉 party")); // Emoji with text
    assert!(!umt_has_no_letters(
        "
      Hello
      World
      😄
      123
    "
    )); // Multiline mixed content
}
