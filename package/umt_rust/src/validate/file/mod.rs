//! File validation module
//! Provides validation for File / Blob instances. In Rust, file / blob content
//! is modeled as raw bytes (`&[u8]`), and any byte slice is structurally valid
//! file content, so validation always succeeds for the statically-typed input.

use super::core::ValidateCoreReturnType;

/// Validates a file value represented as a byte slice
///
/// The TypeScript original checks `value instanceof File || value instanceof Blob`.
/// Rust has no `File` / `Blob` globals, so file content is represented as `&[u8]`.
/// Because any byte slice is structurally valid file content, this always returns
/// `validate: true`, mirroring how the TypeScript validator accepts every `File`.
///
/// # Arguments
/// * `value` - The file content (byte slice) to validate
/// * `message` - Custom error message
///
/// # Returns
/// A `ValidateCoreReturnType` containing the validation result
///
/// # Examples
/// ```
/// use umt_rust::validate::file::umt_validate_file;
///
/// let result = umt_validate_file(b"hello".to_vec(), None);
/// assert!(result.validate);
/// ```
#[inline]
pub fn umt_validate_file(value: Vec<u8>, message: Option<&str>) -> ValidateCoreReturnType<Vec<u8>> {
    ValidateCoreReturnType {
        validate: true,
        message: message.unwrap_or("").to_string(),
        type_value: value,
    }
}

/// Creates a file validator function
///
/// Mirrors the TypeScript `file(message?)` factory, which returns a validator
/// closure for `File` / `Blob` instances. In Rust, file content is represented
/// as `Vec<u8>`, and the returned closure always reports the value as valid.
///
/// # Arguments
/// * `message` - Custom error message for type validation
///
/// # Returns
/// A function that validates file content
///
/// # Examples
/// ```
/// use umt_rust::validate::file::umt_file_validator;
///
/// let validator = umt_file_validator(None);
/// let result = validator(b"hello".to_vec());
/// assert!(result.validate);
/// ```
pub fn umt_file_validator(
    message: Option<String>,
) -> Box<dyn Fn(Vec<u8>) -> ValidateCoreReturnType<Vec<u8>>> {
    Box::new(move |value: Vec<u8>| ValidateCoreReturnType {
        validate: true,
        message: message.clone().unwrap_or_default(),
        type_value: value,
    })
}
