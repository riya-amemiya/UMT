//! Tests for UUID version filtering and strict ASCII matching

use umt_rust::validate::string::umt_uuid;

fn uuid_with_version(version: u8) -> String {
    format!("123e4567-e89b-{version}2d3-a456-426614174000")
}

#[test]
fn test_uuid_rejects_everything_when_all_versions_are_out_of_range() {
    let validator = umt_uuid(Some(vec![0, 8, 99]), None);
    assert!(!(validator.validate)(&uuid_with_version(4)));
    assert!(!(validator.validate)(&uuid_with_version(8)));
    assert!(!(validator.validate)(&uuid_with_version(0)));
    assert!(!(validator.validate)(&uuid_with_version(9)));
}

#[test]
fn test_uuid_rejects_everything_for_an_empty_version_list() {
    let validator = umt_uuid(Some(vec![]), None);
    assert!(!(validator.validate)(&uuid_with_version(4)));
}

#[test]
fn test_uuid_filters_out_invalid_versions_and_keeps_valid_ones() {
    let validator = umt_uuid(Some(vec![4, 99]), None);
    assert!((validator.validate)(&uuid_with_version(4)));
    assert!(!(validator.validate)(&uuid_with_version(9)));
}

#[test]
fn test_uuid_accepts_every_supported_version() {
    let validator = umt_uuid(Some(vec![1, 2, 3, 4, 5, 6, 7]), None);
    for version in 1..=7 {
        assert!((validator.validate)(&uuid_with_version(version)));
    }
    assert!(!(validator.validate)(&uuid_with_version(8)));
}

#[test]
fn test_uuid_only_accepts_requested_versions() {
    let validator = umt_uuid(Some(vec![1, 7]), None);
    assert!((validator.validate)(&uuid_with_version(1)));
    assert!((validator.validate)(&uuid_with_version(7)));
    assert!(!(validator.validate)(&uuid_with_version(4)));
}

#[test]
fn test_uuid_rejects_non_ascii_digits() {
    let validator = umt_uuid(None, None);
    let arabic_digits = char::from_u32(0x0661).unwrap().to_string().repeat(8);
    let value = format!("{arabic_digits}-e89b-42d3-a456-426614174000");
    assert!(!(validator.validate)(&value));
}

#[test]
fn test_uuid_rejects_a_trailing_newline_and_bad_variant() {
    let validator = umt_uuid(None, None);
    assert!(!(validator.validate)(
        &"123e4567-e89b-42d3-a456-426614174000\n".to_string()
    ));
    assert!(!(validator.validate)(
        &"123e4567-e89b-42d3-c456-426614174000".to_string()
    ));
}

#[test]
fn test_uuid_keeps_the_custom_message_when_no_version_is_supported() {
    let validator = umt_uuid(Some(vec![0]), Some("bad uuid".to_string()));
    assert_eq!(validator.message, Some("bad uuid".to_string()));
}
