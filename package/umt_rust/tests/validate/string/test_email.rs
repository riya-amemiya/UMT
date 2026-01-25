//! Tests for email validation

use umt_rust::validate::ParseEmailLevel;
use umt_rust::validate::string::umt_validate_email_validator;

#[test]
fn test_email_with_message() {
    let validator = umt_validate_email_validator(Some("Invalid email format".to_string()), None);

    assert!((validator.validate)(&"user@example.com".to_string()));
    assert!(!(validator.validate)(&"userexample.com".to_string()));
    assert_eq!(validator.message, Some("Invalid email format".to_string()));
}

#[test]
fn test_email_without_message() {
    let validator = umt_validate_email_validator(None, None);

    assert!((validator.validate)(&"user@example.com".to_string()));
    assert!(!(validator.validate)(&"userexample.com".to_string()));
}

#[test]
fn test_email_validates_various_valid_formats() {
    let validator = umt_validate_email_validator(None, None);

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
        assert!(
            (validator.validate)(&email.to_string()),
            "Expected {} to be valid",
            email
        );
    }
}

#[test]
fn test_email_rejects_various_invalid_formats() {
    let validator = umt_validate_email_validator(None, None);

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
        assert!(
            !(validator.validate)(&email.to_string()),
            "Expected {} to be invalid",
            email
        );
    }
}

#[test]
fn test_email_regex_limitations() {
    let validator = umt_validate_email_validator(None, None);

    let buggy_emails = vec![
        "user@example..com",
        "user@.example.com",
        "user@example.com.",
    ];

    for email in buggy_emails {
        assert!(
            !(validator.validate)(&email.to_string()),
            "Expected {} to be invalid",
            email
        );
    }
}

#[test]
fn test_email_edge_cases_and_special_characters() {
    let validator = umt_validate_email_validator(None, None);

    let edge_cases = vec![
        ("user+filter@example.com", true),
        ("user_test@example.com", true),
        ("123456@example.com", true),
    ];

    for (email, expected) in edge_cases {
        assert_eq!(
            (validator.validate)(&email.to_string()),
            expected,
            "Email: {}",
            email
        );
    }
}

#[test]
fn test_email_validates_different_tld_formats() {
    let validator = umt_validate_email_validator(None, None);

    let tld_emails = vec![
        ("user@example.com", true),
        ("user@example.co", true),
        ("user@example.org", true),
        ("user@example.net", true),
        ("user@example.info", true),
    ];

    for (email, expected) in tld_emails {
        assert_eq!(
            (validator.validate)(&email.to_string()),
            expected,
            "Email: {}",
            email
        );
    }
}

#[test]
fn test_email_boundary_length_cases() {
    let validator = umt_validate_email_validator(None, None);

    let short_local = "a@example.com";
    assert!((validator.validate)(&short_local.to_string()));

    let long_local = format!("{}@example.com", "a".repeat(50));
    assert!((validator.validate)(&long_local.to_string()));
}

#[test]
fn test_email_rfc822_obsolete_cases() {
    let validator = umt_validate_email_validator(None, Some(ParseEmailLevel::Rfc822));

    // RFC 822 allowed domains without dots
    assert!((validator.validate)(&"easy@example".to_string()));
}

#[test]
fn test_email_space_related_edge_cases() {
    let validator = umt_validate_email_validator(None, None);

    let space_cases = vec![
        "what about spaces@example.com",
        " maybe-like-this @example.com",
        "fed-up-yet@ example.com ",
    ];

    for email in space_cases {
        assert!(
            !(validator.validate)(&email.to_string()),
            "Expected {} to be invalid",
            email
        );
    }
}

#[test]
fn test_email_comment_syntax() {
    let validator = umt_validate_email_validator(None, None);

    let comment_cases = vec!["normal(wtf is this?)@example.com", "(@)@example.com"];

    for email in comment_cases {
        assert!(
            !(validator.validate)(&email.to_string()),
            "Expected {} to be invalid",
            email
        );
    }
}

#[test]
fn test_email_quoted_string_syntax() {
    let validator = umt_validate_email_validator(None, None);

    let quoted_cases = vec![
        r#"":(){|:&};:"@example.com"#,
        r#"""@example.com"#,
        r#""@"@[@]"#,
    ];

    for email in quoted_cases {
        assert!(
            !(validator.validate)(&email.to_string()),
            "Expected {} to be invalid",
            email
        );
    }
}

#[test]
fn test_email_ipv6_address_literals() {
    let validator = umt_validate_email_validator(None, None);

    let ipv6_cases = vec!["magic@[::1]", "test@[127.0.0.1]"];

    for email in ipv6_cases {
        assert!(
            !(validator.validate)(&email.to_string()),
            "Expected {} to be invalid at basic level",
            email
        );
    }
}
