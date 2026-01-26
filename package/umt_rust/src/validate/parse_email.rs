//! Email parsing and validation utility

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

fn validate_basic_email(email: &str) -> Option<EmailParts> {
    let at_pos = email.find('@')?;
    if at_pos == 0 || at_pos == email.len() - 1 {
        return None;
    }
    // Check no double @
    if email[at_pos + 1..].contains('@') {
        return None;
    }
    let local = &email[..at_pos];
    let domain = &email[at_pos + 1..];

    // Validate local part
    if !local.chars().all(|c| c.is_ascii_alphanumeric() || ".!#$%&'*+/=?^_`{|}~-".contains(c)) {
        return None;
    }

    // Validate domain
    if domain.is_empty() || domain.starts_with('.') || domain.ends_with('.') || domain.starts_with('-') {
        return None;
    }
    for part in domain.split('.') {
        if part.is_empty() || part.len() > 63 {
            return None;
        }
        if !part.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return None;
        }
    }

    Some(EmailParts {
        local: local.to_string(),
        domain: domain.to_string(),
    })
}

fn validate_rfc2822_email(email: &str) -> Option<EmailParts> {
    let parts = validate_basic_email(email)?;
    // RFC2822 requires at least one dot in domain (TLD must be 2+ chars)
    if !parts.domain.contains('.') {
        return None;
    }
    let tld = parts.domain.rsplit('.').next()?;
    if tld.len() < 2 || !tld.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    Some(parts)
}

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

    let parts = match opts.level {
        ParseEmailLevel::Basic | ParseEmailLevel::Rfc822 => validate_basic_email(email),
        ParseEmailLevel::Rfc2822 => validate_rfc2822_email(email),
        ParseEmailLevel::Rfc5321 | ParseEmailLevel::Rfc5322 => validate_basic_email(email),
    };

    match parts {
        Some(parts) => ParseEmailResult {
            valid: true,
            parts: Some(parts),
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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_validate_email() {
        assert!(umt_validate_email("user@example.com"));
        assert!(umt_validate_email("test.user@subdomain.example.org"));
        assert!(!umt_validate_email("invalid-email"));
        assert!(!umt_validate_email("@example.com"));
        assert!(!umt_validate_email("user@"));
    }
}
