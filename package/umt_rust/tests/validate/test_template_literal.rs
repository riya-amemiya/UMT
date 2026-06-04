//! Tests for template literal validation

use umt_rust::validate::{TemplateLiteralPart, umt_template_literal};

fn literal(value: &str) -> TemplateLiteralPart {
    TemplateLiteralPart::Literal(value.to_string())
}

#[test]
fn test_template_literal_accepts_matching_strings() {
    let validator = umt_template_literal(
        &[
            literal("hello, "),
            TemplateLiteralPart::StringTag,
            literal("!"),
        ],
        None,
    );

    assert!(validator("hello, world!").validate);
    assert!(validator("hello, John!").validate);
}

#[test]
fn test_template_literal_rejects_non_matching_strings() {
    let validator = umt_template_literal(
        &[
            literal("hello, "),
            TemplateLiteralPart::StringTag,
            literal("!"),
        ],
        None,
    );

    assert!(!validator("hi, world!").validate);
    assert!(!validator("hello, world").validate);
}

#[test]
fn test_template_literal_returns_configured_message_on_failure() {
    let validator = umt_template_literal(
        &[literal("user-"), TemplateLiteralPart::NumberTag],
        Some("must be user-<number>"),
    );

    let result = validator("user-abc");
    assert!(!result.validate);
    assert_eq!(result.message, "must be user-<number>");
}

#[test]
fn test_template_literal_supports_number_validators() {
    let validator = umt_template_literal(&[literal("item-"), TemplateLiteralPart::NumberTag], None);

    assert!(validator("item-1").validate);
    assert!(validator("item-100").validate);
    assert!(validator("item-1.5").validate);
    assert!(!validator("item-abc").validate);
}

#[test]
fn test_template_literal_supports_boolean_validators() {
    let validator =
        umt_template_literal(&[literal("flag-"), TemplateLiteralPart::BooleanTag], None);

    assert!(validator("flag-true").validate);
    assert!(validator("flag-false").validate);
    assert!(!validator("flag-yes").validate);
}

#[test]
fn test_template_literal_supports_bigint_validators() {
    let validator = umt_template_literal(&[literal("big-"), TemplateLiteralPart::BigIntTag], None);

    assert!(validator("big-1").validate);
    assert!(validator("big-100").validate);
    assert!(!validator("big-abc").validate);
}

#[test]
fn test_template_literal_escapes_special_regex_characters() {
    let validator = umt_template_literal(
        &[literal("[$]"), TemplateLiteralPart::StringTag, literal(".")],
        None,
    );

    assert!(validator("[$]anything.").validate);
    assert!(!validator("xanythingy").validate);
}

#[test]
fn test_template_literal_unknown_tag_is_permissive() {
    let validator =
        umt_template_literal(&[literal("prefix-"), TemplateLiteralPart::UnknownTag], None);

    assert!(validator("prefix-anything").validate);
    assert!(!validator("noprefix").validate);
}

#[test]
fn test_template_literal_preserves_value_in_type_value() {
    let validator = umt_template_literal(&[literal("user-"), TemplateLiteralPart::NumberTag], None);

    let result = validator("user-1");
    assert!(result.validate);
    assert_eq!(result.type_value, "user-1");
    assert_eq!(result.message, "");
}
