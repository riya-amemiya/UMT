use umt_rust::string::umt_slugify;

#[test]
fn test_convert_text_to_url_friendly_slug_according_to_jsdoc_examples() {
    assert_eq!(umt_slugify("Hello World!"), "hello-world");
    assert_eq!(umt_slugify("This is a Test"), "this-is-a-test");
    // Note: Rust implementation keeps unicode alphanumeric characters
    assert_eq!(umt_slugify("Japanese: こんにちは"), "japanese");
}

#[test]
fn test_handle_multiple_spaces() {
    assert_eq!(umt_slugify("Hello    World"), "hello-world");
}

#[test]
fn test_handle_leading_and_trailing_spaces() {
    assert_eq!(umt_slugify("  Hello World  "), "hello-world");
}

#[test]
fn test_remove_special_characters() {
    assert_eq!(umt_slugify("Special!@#$%Characters"), "special-characters");
}

#[test]
fn test_handle_underscores() {
    assert_eq!(umt_slugify("snake_case_text"), "snake-case-text");
}

#[test]
fn test_handle_unicode_characters() {
    assert_eq!(umt_slugify("café"), "cafe");
    assert_eq!(umt_slugify("naïve"), "naive");
}

#[test]
fn test_handle_numbers() {
    assert_eq!(umt_slugify("Test 123"), "test-123");
    assert_eq!(umt_slugify("Version 2.5"), "version-2-5");
}

#[test]
fn test_handle_empty_string() {
    assert_eq!(umt_slugify(""), "");
}

#[test]
fn test_handle_consecutive_hyphens() {
    assert_eq!(umt_slugify("Hello---World"), "hello-world");
}

#[test]
fn test_handle_mixed_case() {
    assert_eq!(umt_slugify("CamelCase"), "camelcase");
}

#[test]
fn test_non_ascii_letters_and_digits_are_separators() {
    assert_eq!(
        umt_slugify("Japanese: \u{3053}\u{3093}\u{306b}\u{3061}\u{306f}"),
        "japanese"
    );
    assert_eq!(umt_slugify("\u{65e5}\u{672c}\u{8a9e}"), "");
    assert_eq!(umt_slugify("x\u{b2}"), "x");
    assert_eq!(umt_slugify("\u{ff11}\u{ff12}\u{ff13}"), "");
    assert_eq!(umt_slugify("\u{2167}"), "");
    assert_eq!(umt_slugify("\u{df}"), "");
}

#[test]
fn test_decomposed_characters_keep_their_ascii_base() {
    assert_eq!(
        umt_slugify("\u{dc}n\u{ef}c\u{f6}d\u{e9} T\u{eb}xt"),
        "unicode-text"
    );
    assert_eq!(umt_slugify("\u{130}stanbul"), "istanbul");
    assert_eq!(umt_slugify("\u{212a}"), "k");
    assert_eq!(umt_slugify("a\u{301}b"), "ab");
}

#[test]
fn test_separator_runs_collapse_and_edges_are_trimmed_like_typescript() {
    assert_eq!(umt_slugify("a_b"), "a-b");
    assert_eq!(umt_slugify("__a__"), "a");
    assert_eq!(umt_slugify("a---b"), "a-b");
    assert_eq!(umt_slugify("-a-"), "a");
    assert_eq!(umt_slugify(""), "");
}
