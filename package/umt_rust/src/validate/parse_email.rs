//! Email parsing and validation utility

use regex::Regex;
use std::sync::LazyLock;

/// Email validation level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseEmailLevel {
    Basic,
    Rfc822,
    Rfc2822,
    Rfc5321,
    Rfc5322,
}

/// Email parsing options
#[derive(Debug, Clone)]
pub struct ParseEmailOptions {
    pub level: ParseEmailLevel,
}

impl Default for ParseEmailOptions {
    fn default() -> Self {
        Self {
            level: ParseEmailLevel::Basic,
        }
    }
}

/// Parsed email parts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailParts {
    pub local: String,
    pub domain: String,
}

/// Email parsing result
#[derive(Debug, Clone)]
pub struct ParseEmailResult {
    pub valid: bool,
    pub parts: Option<EmailParts>,
}

// Pre-compiled regex patterns
static BASIC_EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<local>[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+)@(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*)$").unwrap()
});

static RFC2822_EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<local>[a-zA-Z0-9!#$%&'*/=?^_`{|}~-](?:[a-zA-Z0-9!#$%&'*/=?^_`{|}~.+-]{0,62}[a-zA-Z0-9!#$%&'*/=?^_`{|}~-])?)@(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*\.[a-zA-Z]{2,})$").unwrap()
});

static RFC5321_EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^(?P<local>(?:[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[^"\\]|\\[\s\S]){0,62}"))@(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+|\[(?:(?:[0-9]{1,3}\.){3}[0-9]{1,3}|IPv6:[0-9a-fA-F:]+)\])$"#).unwrap()
});

/// Parses and validates an email address
///
/// # Arguments
/// * `email` - The email address to parse
/// * `options` - Parsing options including validation level
///
/// # Returns
/// A `ParseEmailResult` containing validation status and parsed parts
///
/// # Examples
/// ```
/// use umt_rust::validate::{umt_parse_email, ParseEmailOptions, ParseEmailLevel};
///
/// let result = umt_parse_email("user@example.com", None);
/// assert!(result.valid);
/// assert_eq!(result.parts.unwrap().local, "user");
///
/// let opts = ParseEmailOptions { level: ParseEmailLevel::Rfc2822 };
/// let result = umt_parse_email("user@example.com", Some(opts));
/// assert!(result.valid);
/// ```
#[inline]
pub fn umt_parse_email(email: &str, options: Option<ParseEmailOptions>) -> ParseEmailResult {
    let opts = options.unwrap_or_default();

    let regex = match opts.level {
        ParseEmailLevel::Basic | ParseEmailLevel::Rfc822 => &*BASIC_EMAIL_REGEX,
        ParseEmailLevel::Rfc2822 => &*RFC2822_EMAIL_REGEX,
        ParseEmailLevel::Rfc5321 | ParseEmailLevel::Rfc5322 => &*RFC5321_EMAIL_REGEX,
    };

    match regex.captures(email) {
        Some(caps) => ParseEmailResult {
            valid: true,
            parts: Some(EmailParts {
                local: caps
                    .name("local")
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default(),
                domain: caps
                    .name("domain")
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default(),
            }),
        },
        None => ParseEmailResult {
            valid: false,
            parts: None,
        },
    }
}

/// Validates an email address (simple wrapper)
///
/// # Arguments
/// * `email` - The email address to validate
///
/// # Returns
/// true if the email is valid, false otherwise
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_validate_email;
///
/// assert!(umt_validate_email("user@example.com"));
/// assert!(!umt_validate_email("invalid-email"));
/// ```
#[inline]
pub fn umt_validate_email(email: &str) -> bool {
    umt_parse_email(email, None).valid
}
