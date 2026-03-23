use umt_rust::url::umt_is_absolute_url;

#[test]
fn test_returns_true_for_http_urls() {
    assert!(umt_is_absolute_url("http://example.com"));
    assert!(umt_is_absolute_url("https://example.com"));
}

#[test]
fn test_returns_true_for_other_schemes() {
    assert!(umt_is_absolute_url("ftp://files.example"));
    assert!(umt_is_absolute_url("mailto:user@host"));
    assert!(umt_is_absolute_url("tel:+1234567890"));
    assert!(umt_is_absolute_url("ssh://host"));
}

#[test]
fn test_returns_true_for_custom_schemes() {
    assert!(umt_is_absolute_url("custom+scheme://path"));
    assert!(umt_is_absolute_url("my-app://deep-link"));
    assert!(umt_is_absolute_url("x.y://test"));
}

#[test]
fn test_returns_false_for_relative_paths() {
    assert!(!umt_is_absolute_url("/path/to/page"));
    assert!(!umt_is_absolute_url("relative/path"));
    assert!(!umt_is_absolute_url("./relative"));
    assert!(!umt_is_absolute_url("../parent"));
}

#[test]
fn test_returns_false_for_protocol_relative_urls() {
    assert!(!umt_is_absolute_url("//example.com"));
}

#[test]
fn test_returns_false_for_empty_string() {
    assert!(!umt_is_absolute_url(""));
}

#[test]
fn test_returns_false_for_invalid_scheme_starts() {
    assert!(!umt_is_absolute_url("123://invalid"));
    assert!(!umt_is_absolute_url("+bad://invalid"));
}
