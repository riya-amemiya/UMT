//! Tests for file core validation

use umt_rust::validate::file::{umt_file_validator, umt_validate_file};

#[test]
fn test_validate_file_accepts_byte_content() {
    let result = umt_validate_file(b"hello".to_vec(), None);
    assert!(result.validate);
    assert_eq!(result.type_value, b"hello".to_vec());
}

#[test]
fn test_validate_file_accepts_empty_content() {
    let result = umt_validate_file(Vec::new(), None);
    assert!(result.validate);
    assert!(result.type_value.is_empty());
}

#[test]
fn test_validate_file_with_custom_message() {
    let custom_message = "must be a file";
    let result = umt_validate_file(b"hello".to_vec(), Some(custom_message));
    assert!(result.validate);
    assert_eq!(result.message, custom_message);
}

#[test]
fn test_file_validator_function() {
    let validator = umt_file_validator(None);
    let result = validator(b"hello".to_vec());
    assert!(result.validate);
    assert_eq!(result.type_value, b"hello".to_vec());
}

#[test]
fn test_file_validator_with_message() {
    let validator = umt_file_validator(Some("must be a file".to_string()));
    let result = validator(b"hello".to_vec());
    assert!(result.validate);
    assert_eq!(result.message, "must be a file");
}

// Note: In Rust, file / blob content is represented as a byte slice, which is
// always structurally valid, so we cannot test rejection of non-file values as
// in TypeScript. The type system constrains the input at compile time.
