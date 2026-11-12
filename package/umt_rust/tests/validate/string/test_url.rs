//! Tests for URL string validation

use umt_rust::validate::string::{umt_string_validator, umt_url};

#[test]
fn test_url_accepts_absolute_urls() {
    let validator = umt_string_validator(vec![umt_url(None)], None);
    for value in [
        "https://example.com",
        "http://localhost:3000/path?q=1",
        "ftp://files.example",
        "mailto:user@host",
        "HTTPS://EXAMPLE.COM",
    ] {
        assert!(validator(value).validate, "{value}");
    }
}

#[test]
fn test_url_rejects_relative_or_malformed_urls() {
    let validator = umt_string_validator(vec![umt_url(Some("must be a URL".to_string()))], None);
    for value in [
        "/path/to/page",
        "relative/path",
        "",
        "1http://example.com",
        "//example.com",
    ] {
        let result = validator(value);
        assert!(!result.validate, "{value}");
        assert_eq!(result.message, "must be a URL");
    }
}

#[test]
fn test_url_defaults_the_message_to_empty_string() {
    let validator = umt_string_validator(vec![umt_url(None)], None);
    assert_eq!(validator("example.com").message, "");
}
