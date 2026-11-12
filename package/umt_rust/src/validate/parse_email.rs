//! Email parsing and validation utility

use regex::Regex;
use std::borrow::Cow;
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

const MAX_EMAIL_LENGTH: usize = 320;
const MAX_RFC5321_LENGTH: usize = 256;

const COMMENT_OR_SPACE: &str = concat!(
    r"(?:[\t\n\x0B\x0C\r \x{A0}\x{1680}\x{2000}-\x{200A}\x{2028}\x{2029}\x{202F}\x{205F}\x{3000}\x{FEFF}]",
    r"|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*",
);
const QUOTED: &str = r#""(?:[^"\\]|\\[\s\S]){0,62}""#;
const ATOM_CHAR: &str = r"[A-Za-z0-9_!#$%&'*/=?^`{|}~-]";
const ATOM_PLUS_CHAR: &str = r"[A-Za-z0-9_!#$%&'*+/=?^`{|}~-]";
const LABEL: &str = r"[0-9A-Za-z](?:[0-9A-Za-z-]{0,61}[0-9A-Za-z])?";
const ADDRESS_LITERAL: &str = r"\[(?:(?:[0-9]{1,3}\.){3}[0-9]{1,3}|IPv6:[0-9:A-Fa-f]+)\]";

fn compile(parts: &[&str]) -> Regex {
    Regex::new(&parts.concat()).unwrap()
}

// Pre-compiled regex patterns
static BASIC_EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    compile(&[
        r"^(?P<local>[A-Za-z0-9_!#$%&'*+./=?^`{|}~-]+)@(?P<domain>",
        LABEL,
        r"(?:\.",
        LABEL,
        r")*)$",
    ])
});

static RFC822_EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    compile(&[
        "^",
        COMMENT_OR_SPACE,
        "(?P<local>",
        QUOTED,
        "|",
        ATOM_CHAR,
        r"{1,64}(?:\.",
        ATOM_CHAR,
        "+)*)",
        COMMENT_OR_SPACE,
        "@",
        COMMENT_OR_SPACE,
        "(?P<domain>",
        LABEL,
        r"(?:\.",
        LABEL,
        ")*)",
        COMMENT_OR_SPACE,
        "$",
    ])
});

static RFC2822_EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    compile(&[
        "^(?P<local>",
        ATOM_CHAR,
        r"(?:[A-Za-z0-9_!#$%&'*+./=?^`{|}~-]{0,62}",
        ATOM_CHAR,
        ")?)@(?P<domain>",
        LABEL,
        r"(?:\.",
        LABEL,
        r")*\.[A-Za-z]{2,})$",
    ])
});

static RFC5321_EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    compile(&[
        "^(?P<local>(?:",
        ATOM_PLUS_CHAR,
        r"+(?:\.",
        ATOM_PLUS_CHAR,
        "+)*|",
        QUOTED,
        "))@(?P<domain>",
        LABEL,
        r"(?:\.",
        LABEL,
        ")+|",
        ADDRESS_LITERAL,
        ")$",
    ])
});

static RFC5322_EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    compile(&[
        "^",
        COMMENT_OR_SPACE,
        "(?P<local>",
        QUOTED,
        r"(?:\.",
        QUOTED,
        ")*|",
        QUOTED,
        r"(?:\.",
        ATOM_PLUS_CHAR,
        "{1,64})+|",
        ATOM_PLUS_CHAR,
        r"{1,64}(?:\.",
        ATOM_PLUS_CHAR,
        r"{1,64})*(?:\.",
        QUOTED,
        ")+|",
        ATOM_PLUS_CHAR,
        "{1,64}(?:",
        COMMENT_OR_SPACE,
        r"\.",
        COMMENT_OR_SPACE,
        ATOM_PLUS_CHAR,
        "{1,64})*)",
        COMMENT_OR_SPACE,
        "@",
        COMMENT_OR_SPACE,
        "(?P<domain>",
        LABEL,
        "(?:",
        COMMENT_OR_SPACE,
        r"\.",
        COMMENT_OR_SPACE,
        LABEL,
        ")+|",
        ADDRESS_LITERAL,
        ")",
        COMMENT_OR_SPACE,
        "$",
    ])
});

fn is_single_line(email: &str) -> bool {
    !email.is_empty()
        && !email
            .chars()
            .any(|c| matches!(c as u32, 0x0A | 0x0D | 0x2028 | 0x2029))
}

fn has_short_local_part(email: &str) -> bool {
    email
        .find('@')
        .is_some_and(|index| (1..=64).contains(&email[..index].chars().count()))
}

fn to_utf16_units(email: &str) -> Cow<'_, str> {
    if email.chars().all(|c| c.len_utf16() == 1) {
        return Cow::Borrowed(email);
    }
    Cow::Owned(
        email
            .chars()
            .flat_map(|c| {
                std::iter::repeat_n(
                    if c.len_utf16() == 1 { c } else { '\u{FFFD}' },
                    c.len_utf16(),
                )
            })
            .collect(),
    )
}

fn byte_offset_at_unit(email: &str, unit: usize) -> usize {
    email
        .char_indices()
        .scan(0, |seen, (index, c)| {
            let before = *seen;
            *seen += c.len_utf16();
            Some((before, index))
        })
        .find(|(before, _)| *before >= unit)
        .map_or(email.len(), |(_, index)| index)
}

fn source_slice(email: &str, units: &str, range: std::ops::Range<usize>) -> String {
    let start = byte_offset_at_unit(email, units[..range.start].chars().count());
    let end = byte_offset_at_unit(email, units[..range.end].chars().count());
    email[start..end].to_string()
}

/// Parses and validates an email address
///
/// Inputs longer than 320 UTF-16 code units are rejected before any pattern
/// is evaluated, matching the TypeScript implementation. Every length limit
/// counts UTF-16 code units, so a character outside the Basic Multilingual
/// Plane counts as two.
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
    let invalid = ParseEmailResult {
        valid: false,
        parts: None,
    };

    let units = to_utf16_units(email);
    let unit_count = units.chars().count();
    if unit_count > MAX_EMAIL_LENGTH {
        return invalid;
    }

    let (regex, precondition) = match opts.level {
        ParseEmailLevel::Basic => (&*BASIC_EMAIL_REGEX, true),
        ParseEmailLevel::Rfc822 => (&*RFC822_EMAIL_REGEX, true),
        ParseEmailLevel::Rfc2822 => (
            &*RFC2822_EMAIL_REGEX,
            is_single_line(&units) && !units.contains(".."),
        ),
        ParseEmailLevel::Rfc5321 => (
            &*RFC5321_EMAIL_REGEX,
            is_single_line(&units)
                && unit_count <= MAX_RFC5321_LENGTH
                && has_short_local_part(&units)
                && !units.contains(".."),
        ),
        ParseEmailLevel::Rfc5322 => (&*RFC5322_EMAIL_REGEX, is_single_line(&units)),
    };

    if !precondition {
        return invalid;
    }

    match regex.captures(&units) {
        Some(caps) => ParseEmailResult {
            valid: true,
            parts: Some(EmailParts {
                local: caps
                    .name("local")
                    .map(|m| source_slice(email, &units, m.range()))
                    .unwrap_or_default(),
                domain: caps
                    .name("domain")
                    .map(|m| source_slice(email, &units, m.range()))
                    .unwrap_or_default(),
            }),
        },
        None => invalid,
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
