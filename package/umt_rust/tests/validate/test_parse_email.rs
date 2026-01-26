//! Tests for parse_email function

use umt_rust::validate::{ParseEmailLevel, ParseEmailOptions, umt_parse_email, umt_validate_email};

// Basic email validation tests
#[test]
fn test_parse_email_valid_basic_email() {
    let result = umt_parse_email("user@example.com", None);
    assert!(result.valid);
    let parts = result.parts.unwrap();
    assert_eq!(parts.local, "user");
    assert_eq!(parts.domain, "example.com");
}

#[test]
fn test_parse_email_invalid_email() {
    let result = umt_parse_email("userexample.com", None);
    assert!(!result.valid);
    assert!(result.parts.is_none());
}

#[test]
fn test_parse_email_various_valid_formats() {
    let valid_emails = vec![
        "test@example.com",
        "user.name@example.com",
        "user+tag@example.com",
        "user-name@example.com",
        "test123@example.com",
        "a@example.com",
        "very.long.email.address@example.com",
        "user_name@example.com",
        "123@example.com",
        "test.email+tag+sorting@example.com",
    ];

    for email in valid_emails {
        let result = umt_parse_email(email, None);
        assert!(result.valid, "Expected {} to be valid", email);
    }
}

#[test]
fn test_parse_email_various_invalid_formats() {
    let invalid_emails = vec![
        "plainaddress",
        "@example.com",
        "user@",
        "user.example.com",
        "user@@example.com",
        "user name@example.com",
        "user@example .com",
        "user@example,com",
        "",
        " ",
    ];

    for email in invalid_emails {
        let result = umt_parse_email(email, None);
        assert!(!result.valid, "Expected {} to be invalid", email);
    }
}

#[test]
fn test_parse_email_different_tld_formats() {
    let tld_emails = vec![
        ("user@example.com", true),
        ("user@example.co", true),
        ("user@example.org", true),
        ("user@example.net", true),
        ("user@example.info", true),
    ];

    for (email, expected_valid) in tld_emails {
        let result = umt_parse_email(email, None);
        assert_eq!(result.valid, expected_valid, "Email: {}", email);
    }
}

#[test]
fn test_parse_email_boundary_length_cases() {
    let short_local = "a@example.com";
    let result = umt_parse_email(short_local, None);
    assert!(result.valid);

    let long_local = format!("{}@example.com", "a".repeat(50));
    let result = umt_parse_email(&long_local, None);
    assert!(result.valid);
}

#[test]
fn test_parse_email_space_cases() {
    let space_cases = vec![
        "what about spaces@example.com",
        " maybe-like-this @example.com",
        "fed-up-yet@ example.com ",
    ];

    for email in space_cases {
        let result = umt_parse_email(email, None);
        assert!(!result.valid, "Expected {} to be invalid", email);
    }
}

#[test]
fn test_parse_email_comment_syntax() {
    let comment_cases = vec!["normal(wtf is this?)@example.com", "(@)@example.com"];

    for email in comment_cases {
        let result = umt_parse_email(email, None);
        assert!(!result.valid, "Expected {} to be invalid", email);
    }
}

#[test]
fn test_parse_email_quoted_string_syntax() {
    let quoted_cases = vec![
        r#"":(){|:&};:"@example.com"#,
        r#"""@example.com"#,
        r#""@"@[@]"#,
    ];

    for email in quoted_cases {
        let result = umt_parse_email(email, None);
        assert!(!result.valid, "Expected {} to be invalid", email);
    }
}

#[test]
fn test_parse_email_ipv6_address_literals() {
    let ipv6_cases = vec!["magic@[::1]", "test@[127.0.0.1]"];

    for email in ipv6_cases {
        let result = umt_parse_email(email, None);
        assert!(
            !result.valid,
            "Expected {} to be invalid at basic level",
            email
        );
    }
}

// RFC level tests
#[test]
fn test_parse_email_different_rfc_levels() {
    let opts_basic = ParseEmailOptions {
        level: ParseEmailLevel::Basic,
    };
    let opts_rfc822 = ParseEmailOptions {
        level: ParseEmailLevel::Rfc822,
    };
    let opts_rfc2822 = ParseEmailOptions {
        level: ParseEmailLevel::Rfc2822,
    };
    let opts_rfc5321 = ParseEmailOptions {
        level: ParseEmailLevel::Rfc5321,
    };
    let opts_rfc5322 = ParseEmailOptions {
        level: ParseEmailLevel::Rfc5322,
    };

    let email = "test@example.com";
    assert!(umt_parse_email(email, Some(opts_basic)).valid);
    assert!(umt_parse_email(email, Some(opts_rfc822)).valid);
    assert!(umt_parse_email(email, Some(opts_rfc2822)).valid);
    assert!(umt_parse_email(email, Some(opts_rfc5321)).valid);
    assert!(umt_parse_email(email, Some(opts_rfc5322)).valid);
}

#[test]
fn test_parse_email_rfc822_allows_domains_without_tld() {
    let opts = ParseEmailOptions {
        level: ParseEmailLevel::Rfc822,
    };
    assert!(umt_parse_email("user@localhost", Some(opts.clone())).valid);
    assert!(umt_parse_email("user@example", Some(opts)).valid);
}

#[test]
fn test_parse_email_extracts_parts_correctly() {
    let result = umt_parse_email("user@example.com", None);
    assert!(result.valid);
    let parts = result.parts.unwrap();
    assert_eq!(parts.local, "user");
    assert_eq!(parts.domain, "example.com");

    let result2 = umt_parse_email("test.user@sub.example.com", None);
    assert!(result2.valid);
    let parts2 = result2.parts.unwrap();
    assert_eq!(parts2.local, "test.user");
    assert_eq!(parts2.domain, "sub.example.com");

    let result3 = umt_parse_email("invalid", None);
    assert!(!result3.valid);
    assert!(result3.parts.is_none());
}

// Simple wrapper function tests
#[test]
fn test_validate_email() {
    assert!(umt_validate_email("user@example.com"));
    assert!(umt_validate_email("test.user@subdomain.example.org"));
    assert!(!umt_validate_email("invalid-email"));
    assert!(!umt_validate_email("@example.com"));
    assert!(!umt_validate_email("user@"));
}

#[test]
fn test_parse_email_rejects_completely_invalid_strings() {
    let invalid_strings = vec![
        "not-an-email-at-all",
        "12345",
        "!@#$%^&*()",
        "random string with spaces",
        "just-text",
    ];

    for s in invalid_strings {
        let result = umt_parse_email(s, None);
        assert!(!result.valid, "Expected {} to be invalid", s);
    }
}

#[test]
fn test_parse_email_malformed_domains() {
    let malformed = vec![
        "user@",
        "user@.",
        "user@..",
        "user@...",
        "user@.domain.example.com",
        "user@domain..example.com",
    ];

    for email in malformed {
        let result = umt_parse_email(email, None);
        assert!(!result.valid, "Expected {} to be invalid", email);
    }
}

use umt_rust::validate::*;

#[test]
fn test_parse_email_basic() {
    let result = umt_parse_email("user@example.com", None);
    assert!(result.valid);
    let parts = result.parts.unwrap();
    assert_eq!(parts.local, "user");
    assert_eq!(parts.domain, "example.com");
}

#[test]
fn test_parse_email_invalid() {
    let result = umt_parse_email("invalid-email", None);
    assert!(!result.valid);
    assert!(result.parts.is_none());
}

#[test]
fn test_parse_email_rfc2822() {
    let opts = ParseEmailOptions {
        level: ParseEmailLevel::Rfc2822,
    };
    let result = umt_parse_email("user@example.com", Some(opts));
    assert!(result.valid);
}
