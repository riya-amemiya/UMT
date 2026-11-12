//! Tests for template literal fragments matching the JavaScript regex semantics

use umt_rust::validate::{TemplateLiteralPart, umt_template_literal};

fn literal(value: &str) -> TemplateLiteralPart {
    TemplateLiteralPart::Literal(value.to_string())
}

fn with_char(prefix: &str, code: u32, suffix: &str) -> String {
    format!("{}{}{}", prefix, char::from_u32(code).unwrap(), suffix)
}

const LINE_TERMINATORS: [u32; 4] = [0x0A, 0x0D, 0x2028, 0x2029];

#[test]
fn test_template_literal_number_tag_only_accepts_ascii_digits() {
    let validator = umt_template_literal(&[literal("item-"), TemplateLiteralPart::NumberTag], None);
    assert!(validator("item-12").validate);
    assert!(validator("item--1.5").validate);
    assert!(validator("item-.5").validate);
    assert!(validator("item-5.").validate);
    assert!(!validator(&with_char("item-", 0x0661, "")).validate);
    assert!(!validator(&with_char("item-", 0xFF11, "")).validate);
    assert!(!validator(&with_char("item-1.", 0x0662, "")).validate);
}

#[test]
fn test_template_literal_bigint_tag_only_accepts_ascii_digits() {
    let validator = umt_template_literal(&[literal("id-"), TemplateLiteralPart::BigIntTag], None);
    assert!(validator("id-12").validate);
    assert!(validator("id--12").validate);
    assert!(!validator("id-1.5").validate);
    assert!(!validator(&with_char("id-", 0x0661, "")).validate);
}

#[test]
fn test_template_literal_string_tag_does_not_cross_line_terminators() {
    let validator = umt_template_literal(
        &[literal("a"), TemplateLiteralPart::StringTag, literal("b")],
        None,
    );
    assert!(validator("ab").validate);
    assert!(validator("a b").validate);
    assert!(validator(&with_char("a", 0x3042, "b")).validate);
    for code in LINE_TERMINATORS {
        assert!(!validator(&with_char("a", code, "b")).validate);
    }
}

#[test]
fn test_template_literal_unknown_tag_does_not_cross_line_terminators() {
    let validator = umt_template_literal(
        &[literal("a"), TemplateLiteralPart::UnknownTag, literal("b")],
        None,
    );
    assert!(validator("axb").validate);
    assert!(!validator("ab").validate);
    for code in LINE_TERMINATORS {
        assert!(!validator(&with_char("a", code, "b")).validate);
    }
}

#[test]
fn test_template_literal_boolean_tag_is_case_sensitive() {
    let validator = umt_template_literal(&[TemplateLiteralPart::BooleanTag], Some("bad"));
    assert!(validator("true").validate);
    assert!(validator("false").validate);
    let rejected = validator("True");
    assert!(!rejected.validate);
    assert_eq!(rejected.message, "bad");
}
