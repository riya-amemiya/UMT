//! Tests for UUID validation

use umt_rust::validate::string::umt_uuid;

#[test]
fn test_uuid_validates_valid_uuids() {
    let validator = umt_uuid(None, None);
    assert!((validator.validate)(
        &"123e4567-e89b-42d3-a456-426614174000".to_string()
    ));
    assert!((validator.validate)(
        &"3A86B1AB-237E-473D-A38F-06AA6A2A4783".to_string()
    ));
    assert!((validator.validate)(
        &"B86D03E7-7B2D-4710-897B-77D05A1F0B4B".to_string()
    ));
    assert!((validator.validate)(
        &"573F42D5-1734-45E1-9D4D-AA43E6DF697E".to_string()
    ));
    assert!((validator.validate)(
        &"D1643B1F-DCEC-46A0-9196-D2C9B8446601".to_string()
    ));
    assert!((validator.validate)(
        &"44046745-283C-4507-955E-430DAAD16189".to_string()
    ));
    assert!((validator.validate)(
        &"7185AF69-0153-4C42-B63F-368291A74AAF".to_string()
    ));
    assert!((validator.validate)(
        &"ADD5ABC1-54B8-4D5D-A8A0-ABA55089A1CF".to_string()
    ));
    assert!((validator.validate)(
        &"0C338BD0-768B-4510-95D0-0D1571B08405".to_string()
    ));
    assert!((validator.validate)(
        &"C48EACCC-CEA3-4C4D-B669-0D1669D4DBBF".to_string()
    ));
    assert!((validator.validate)(
        &"A95E3F62-2D0E-4257-95D3-B239AF08115D".to_string()
    ));
}

#[test]
fn test_uuid_validates_uuid_without_hyphens() {
    let validator = umt_uuid(None, None);
    assert!((validator.validate)(
        &"A95E3F622D0E425795D3B239AF08115D".to_string()
    ));
}

#[test]
fn test_uuid_rejects_invalid_uuids() {
    let validator = umt_uuid(None, None);
    // Invalid character at the end
    assert!(!(validator.validate)(
        &"123e4567-e89b-42d3-a456-42661417400Z".to_string()
    ));
    // Invalid version (9)
    assert!(!(validator.validate)(
        &"123e4567-e89b-92d3-a456-426614174000".to_string()
    ));
}

#[test]
fn test_uuid_with_custom_message() {
    let custom_message = "Invalid UUID format";
    let validator = umt_uuid(Some(vec![4]), Some(custom_message.to_string()));

    let valid_uuid = "123e4567-e89b-42d3-a456-426614174000";
    assert!((validator.validate)(&valid_uuid.to_string()));

    let invalid_uuid = "123e4567-e89b-12d3-a456-42661417400Z";
    assert!(!(validator.validate)(&invalid_uuid.to_string()));
    assert_eq!(validator.message, Some(custom_message.to_string()));
}

#[test]
fn test_uuid_with_mixed_case_letters() {
    let validator = umt_uuid(None, None);
    assert!((validator.validate)(
        &"123e4567-E89b-42D3-a456-426614174000".to_string()
    ));
}

#[test]
fn test_uuid_validates_different_versions() {
    let validator = umt_uuid(Some(vec![1, 2, 3, 4, 5]), None);
    // Version 1
    assert!((validator.validate)(
        &"123e4567-e89b-12d3-a456-426614174000".to_string()
    ));
    // Version 2
    assert!((validator.validate)(
        &"123e4567-e89b-22d3-a456-426614174000".to_string()
    ));
    // Version 3
    assert!((validator.validate)(
        &"123e4567-e89b-32d3-a456-426614174000".to_string()
    ));
    // Version 5
    assert!((validator.validate)(
        &"123e4567-e89b-52d3-a456-426614174000".to_string()
    ));
}

#[test]
fn test_uuid_rejects_completely_invalid_formats() {
    let validator = umt_uuid(None, None);
    assert!(!(validator.validate)(
        &"completely-invalid-format".to_string()
    ));
}

#[test]
fn test_uuid_rejects_empty_strings() {
    let validator = umt_uuid(None, None);
    assert!(!(validator.validate)(&"".to_string()));
}

#[test]
fn test_uuid_rejects_insufficient_length() {
    let validator = umt_uuid(None, None);
    assert!(!(validator.validate)(
        &"123e4567-e89b-42d3-a456-42661417400".to_string()
    ));
}

#[test]
fn test_uuid_rejects_excessive_length() {
    let validator = umt_uuid(None, None);
    assert!(!(validator.validate)(
        &"123e4567-e89b-42d3-a456-4266141740001234".to_string()
    ));
}
