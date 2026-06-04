//! Template literal validation module
//!
//! Provides a validator that checks whether a string matches a template
//! literal pattern composed of string fragments and primitive validator tags
//! (string / number / boolean / bigint). The runtime check is performed by an
//! auto-generated regular expression assembled from the parts.

use regex::Regex;

use super::core::ValidateCoreReturnType;

/// A single part of a template literal definition.
///
/// Each part is either a string literal that must appear verbatim, or a
/// primitive validator tag whose accepted shape is converted to a regex
/// fragment at construction time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateLiteralPart {
    /// A literal string fragment that must appear verbatim.
    Literal(String),
    /// A `string` validator, contributing a non-greedy any-character fragment.
    StringTag,
    /// A `number` validator, contributing a signed integer or decimal fragment.
    NumberTag,
    /// A `boolean` validator, contributing a `true`/`false` fragment.
    BooleanTag,
    /// A `bigint` validator, contributing a signed integer fragment.
    BigIntTag,
    /// A validator with no detectable tag, contributing a permissive fragment.
    UnknownTag,
}

/// Escapes characters that carry special meaning inside a regular expression.
///
/// Mirrors the TypeScript `escapeRegex` helper, which escapes the character
/// class `[$()*+.?[\\\]^{|}]`.
fn escape_regex(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        if matches!(
            ch,
            '$' | '(' | ')' | '*' | '+' | '.' | '?' | '[' | '\\' | ']' | '^' | '{' | '|' | '}'
        ) {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    escaped
}

/// Returns the regex fragment that matches a given primitive validator tag.
fn tag_to_pattern(part: &TemplateLiteralPart) -> &'static str {
    match part {
        TemplateLiteralPart::StringTag => ".*?",
        TemplateLiteralPart::NumberTag => r"-?(?:\d+\.\d+|\d+(?:\.\d*)?|\.\d+)",
        TemplateLiteralPart::BooleanTag => "(?:true|false)",
        TemplateLiteralPart::BigIntTag => r"-?\d+",
        TemplateLiteralPart::UnknownTag => ".+?",
        TemplateLiteralPart::Literal(_) => "",
    }
}

/// Creates a validator that checks whether a string matches a template literal
/// pattern. Each part is either a string literal that must appear verbatim or a
/// primitive validator tag (string / number / boolean / bigint) that
/// contributes a regex fragment.
///
/// # Arguments
///
/// * `parts` - Slice of literal strings and primitive validator tags
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A validator function that returns a `ValidateCoreReturnType<String>`
///
/// # Examples
///
/// ```
/// use umt_rust::validate::{umt_template_literal, TemplateLiteralPart};
///
/// let validator = umt_template_literal(
///     &[
///         TemplateLiteralPart::Literal("item-".to_string()),
///         TemplateLiteralPart::NumberTag,
///     ],
///     None,
/// );
/// assert!(validator("item-100").validate);
/// assert!(!validator("item-abc").validate);
/// ```
#[inline]
pub fn umt_template_literal(
    parts: &[TemplateLiteralPart],
    message: Option<&str>,
) -> impl Fn(&str) -> ValidateCoreReturnType<String> + use<> {
    let mut pattern = String::from("^");
    for part in parts {
        match part {
            TemplateLiteralPart::Literal(literal) => pattern.push_str(&escape_regex(literal)),
            tag => {
                pattern.push_str("(?:");
                pattern.push_str(tag_to_pattern(tag));
                pattern.push(')');
            }
        }
    }
    pattern.push('$');

    // The assembled pattern only contains escaped literals and fixed fragments,
    // so compilation cannot fail; fall back to a never-matching regex anyway.
    let regex = Regex::new(&pattern).unwrap_or_else(|_| Regex::new(r"$.^").unwrap());
    let message = message.unwrap_or("").to_string();

    move |value: &str| {
        if regex.is_match(value) {
            ValidateCoreReturnType {
                validate: true,
                message: String::new(),
                type_value: value.to_string(),
            }
        } else {
            ValidateCoreReturnType {
                validate: false,
                message: message.clone(),
                type_value: value.to_string(),
            }
        }
    }
}
