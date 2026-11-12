use umt_rust::url::umt_is_absolute_url;

#[test]
fn test_is_absolute_url_parity_scheme_letters_are_ascii_only() {
    assert!(!umt_is_absolute_url("\u{017F}:"));
    assert!(!umt_is_absolute_url("\u{212A}:x"));
    assert!(!umt_is_absolute_url("\u{0130}:"));
    assert!(!umt_is_absolute_url("a\u{0131}:"));
    assert!(!umt_is_absolute_url("a\u{0130}:"));
    assert!(!umt_is_absolute_url("a\u{017F}:"));
    assert!(!umt_is_absolute_url("a\u{212A}:"));
}

#[test]
fn test_is_absolute_url_parity_scheme_digits_are_ascii_only() {
    assert!(!umt_is_absolute_url("a\u{0663}:"));
    assert!(umt_is_absolute_url("A1+.-:"));
}

#[test]
fn test_is_absolute_url_parity_minimal_and_degenerate_inputs() {
    assert!(umt_is_absolute_url("a:"));
    assert!(!umt_is_absolute_url(":"));
    assert!(!umt_is_absolute_url("a\n:"));
    assert!(!umt_is_absolute_url("\nhttp:"));
}
