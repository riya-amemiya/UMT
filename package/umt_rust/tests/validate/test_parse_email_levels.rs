//! Tests for parse_email level-specific behavior and input limits

use umt_rust::validate::{ParseEmailLevel, ParseEmailOptions, umt_parse_email};

const LEVELS: [ParseEmailLevel; 5] = [
    ParseEmailLevel::Basic,
    ParseEmailLevel::Rfc822,
    ParseEmailLevel::Rfc2822,
    ParseEmailLevel::Rfc5321,
    ParseEmailLevel::Rfc5322,
];

fn is_valid(email: &str, level: ParseEmailLevel) -> bool {
    umt_parse_email(email, Some(ParseEmailOptions { level })).valid
}

fn parts(email: &str, level: ParseEmailLevel) -> Option<(String, String)> {
    umt_parse_email(email, Some(ParseEmailOptions { level }))
        .parts
        .map(|parts| (parts.local, parts.domain))
}

fn repeated_char(code: u32, count: usize) -> String {
    char::from_u32(code).unwrap().to_string().repeat(count)
}

#[test]
fn test_parse_email_rejects_inputs_longer_than_320_code_units() {
    let labels = |count: usize| vec!["b".repeat(60); count].join(".");
    let accepted = format!("{}@{}.com", "a".repeat(64), labels(4));
    assert!(accepted.len() <= 320);
    assert!(is_valid(&accepted, ParseEmailLevel::Basic));

    let rejected = format!("{}@{}.com", "a".repeat(64), labels(5));
    assert!(rejected.len() > 320);
    for level in LEVELS {
        let result = umt_parse_email(&rejected, Some(ParseEmailOptions { level }));
        assert!(!result.valid);
        assert!(result.parts.is_none());
    }
}

#[test]
fn test_parse_email_measures_the_limit_in_utf16_code_units() {
    let accepted = format!("({})a@b.cd", repeated_char(0x3042, 300));
    assert!(accepted.len() > 320);
    assert!(is_valid(&accepted, ParseEmailLevel::Rfc822));

    let rejected = format!("({})a@b.cd", repeated_char(0x1F600, 158));
    assert!(rejected.chars().count() < 320);
    assert!(!is_valid(&rejected, ParseEmailLevel::Rfc822));
}

#[test]
fn test_parse_email_rfc822_accepts_comments_and_quoted_local_parts() {
    assert_eq!(
        parts("(comment)user@example.com", ParseEmailLevel::Rfc822),
        Some(("user".to_string(), "example.com".to_string()))
    );
    assert_eq!(
        parts(" user @ example.com ", ParseEmailLevel::Rfc822),
        Some(("user".to_string(), "example.com".to_string()))
    );
    assert_eq!(
        parts("\"john doe\"@example.com", ParseEmailLevel::Rfc822),
        Some(("\"john doe\"".to_string(), "example.com".to_string()))
    );
    assert!(is_valid(
        "(a(nested)b)user@example.com",
        ParseEmailLevel::Rfc822
    ));
    assert!(is_valid("user@example.com\n", ParseEmailLevel::Rfc822));
}

#[test]
fn test_parse_email_rfc822_rejects_plus_and_unbalanced_comments() {
    assert!(!is_valid("user+tag@example.com", ParseEmailLevel::Rfc822));
    assert!(!is_valid(
        "(comment user@example.com",
        ParseEmailLevel::Rfc822
    ));
    assert!(!is_valid("user@[127.0.0.1]", ParseEmailLevel::Rfc822));
}

#[test]
fn test_parse_email_rfc822_uses_javascript_whitespace() {
    let with_prefix = |code: u32| format!("{}user@example.com", char::from_u32(code).unwrap());
    assert!(is_valid(&with_prefix(0xFEFF), ParseEmailLevel::Rfc822));
    assert!(is_valid(&with_prefix(0xA0), ParseEmailLevel::Rfc822));
    assert!(!is_valid(&with_prefix(0x85), ParseEmailLevel::Rfc822));
}

#[test]
fn test_parse_email_rfc2822_requires_a_top_level_domain() {
    assert!(is_valid("user+tag@example.com", ParseEmailLevel::Rfc2822));
    assert!(!is_valid("user@localhost", ParseEmailLevel::Rfc2822));
    assert!(!is_valid("user@example.c", ParseEmailLevel::Rfc2822));
    assert!(!is_valid("user@example.c0m", ParseEmailLevel::Rfc2822));
}

#[test]
fn test_parse_email_rfc2822_rejects_consecutive_dots_and_edge_dots() {
    assert!(!is_valid("a..b@example.com", ParseEmailLevel::Rfc2822));
    assert!(!is_valid(".user@example.com", ParseEmailLevel::Rfc2822));
    assert!(!is_valid("user.@example.com", ParseEmailLevel::Rfc2822));
    assert!(is_valid("a.b@example.com", ParseEmailLevel::Rfc2822));
    assert!(!is_valid("", ParseEmailLevel::Rfc2822));
    assert!(!is_valid("user@example.com\n", ParseEmailLevel::Rfc2822));
}

#[test]
fn test_parse_email_rfc5321_accepts_address_literals_and_quoted_local_parts() {
    assert_eq!(
        parts("user@[192.168.1.1]", ParseEmailLevel::Rfc5321),
        Some(("user".to_string(), "[192.168.1.1]".to_string()))
    );
    assert!(is_valid("user@[IPv6:::1]", ParseEmailLevel::Rfc5321));
    assert!(is_valid(
        "\"john doe\"@example.com",
        ParseEmailLevel::Rfc5321
    ));
    assert!(!is_valid("user@localhost", ParseEmailLevel::Rfc5321));
    assert!(!is_valid(
        "(comment)user@example.com",
        ParseEmailLevel::Rfc5321
    ));
}

#[test]
fn test_parse_email_rfc5321_limits_the_local_part_and_total_length() {
    assert!(is_valid(
        &format!("{}@example.com", "a".repeat(64)),
        ParseEmailLevel::Rfc5321
    ));
    assert!(!is_valid(
        &format!("{}@example.com", "a".repeat(65)),
        ParseEmailLevel::Rfc5321
    ));
    assert!(!is_valid("@example.com", ParseEmailLevel::Rfc5321));
    assert!(!is_valid("userexample.com", ParseEmailLevel::Rfc5321));

    let labels = |count: usize| vec!["b".repeat(60); count].join(".");
    let within = format!("a@{}.com", labels(4));
    assert!(within.len() <= 256);
    assert!(is_valid(&within, ParseEmailLevel::Rfc5321));
    let beyond = format!("a@{}.com", labels(5));
    assert!(beyond.len() > 256 && beyond.len() <= 320);
    assert!(!is_valid(&beyond, ParseEmailLevel::Rfc5321));
    assert!(is_valid(&beyond, ParseEmailLevel::Basic));
}

#[test]
fn test_parse_email_rfc5321_rejects_consecutive_dots_and_line_terminators() {
    assert!(!is_valid("a..b@example.com", ParseEmailLevel::Rfc5321));
    assert!(!is_valid("\"a..b\"@example.com", ParseEmailLevel::Rfc5321));
    assert!(!is_valid("\"a\nb\"@example.com", ParseEmailLevel::Rfc5321));
    assert!(is_valid("\"a b\"@example.com", ParseEmailLevel::Rfc5321));
    assert!(!is_valid("", ParseEmailLevel::Rfc5321));
}

#[test]
fn test_parse_email_rfc5322_accepts_folding_and_mixed_local_parts() {
    assert_eq!(
        parts(" (c) user (d) @ example . com ", ParseEmailLevel::Rfc5322),
        Some(("user".to_string(), "example . com".to_string()))
    );
    assert!(is_valid(
        "\"a\".\"b\"@example.com",
        ParseEmailLevel::Rfc5322
    ));
    assert!(is_valid("\"a\".b.c@example.com", ParseEmailLevel::Rfc5322));
    assert!(is_valid("a.b.\"c\"@example.com", ParseEmailLevel::Rfc5322));
    assert!(is_valid("a . b@example.com", ParseEmailLevel::Rfc5322));
    assert!(is_valid("user+tag@[IPv6:::1]", ParseEmailLevel::Rfc5322));
    assert!(is_valid("user@[10.0.0.1]", ParseEmailLevel::Rfc5322));
}

#[test]
fn test_parse_email_rfc5322_rejects_single_label_domains_and_line_terminators() {
    assert!(!is_valid("user@localhost", ParseEmailLevel::Rfc5322));
    assert!(!is_valid("", ParseEmailLevel::Rfc5322));
    for code in [0x0A, 0x0D, 0x2028, 0x2029] {
        let email = format!("{}user@example.com", char::from_u32(code).unwrap());
        assert!(!is_valid(&email, ParseEmailLevel::Rfc5322));
    }
}

#[test]
fn test_parse_email_basic_rejects_a_trailing_newline() {
    assert!(!is_valid("user@example.com\n", ParseEmailLevel::Basic));
    assert!(is_valid("user@localhost", ParseEmailLevel::Basic));
}

#[test]
fn test_parse_email_counts_quoted_characters_in_utf16_code_units() {
    let accepted = format!("\"{}\"@example.com", repeated_char(0x1F600, 31));
    let rejected = format!("\"{}\"@example.com", repeated_char(0x1F600, 32));
    for level in [
        ParseEmailLevel::Rfc822,
        ParseEmailLevel::Rfc5321,
        ParseEmailLevel::Rfc5322,
    ] {
        assert_eq!(
            parts(&accepted, level),
            Some((
                format!("\"{}\"", repeated_char(0x1F600, 31)),
                "example.com".to_string()
            ))
        );
        assert!(!is_valid(&rejected, level));
    }
}

#[test]
fn test_parse_email_returns_parts_located_after_a_non_bmp_character() {
    let face = repeated_char(0x1F600, 1);
    let email = format!("({face})user@example ({face}) . com");
    assert_eq!(
        parts(&email, ParseEmailLevel::Rfc5322),
        Some(("user".to_string(), format!("example ({face}) . com")))
    );
}

#[test]
fn test_parse_email_keeps_bmp_only_non_ascii_input_unchanged() {
    let email = format!("({})user@example.com", repeated_char(0x3042, 1));
    assert_eq!(
        parts(&email, ParseEmailLevel::Rfc822),
        Some(("user".to_string(), "example.com".to_string()))
    );
}
