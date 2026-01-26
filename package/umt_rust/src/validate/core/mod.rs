//! Core validation module
//! Provides base validation types and the core validation function

use std::fmt::{Debug, Display};
use std::sync::Arc;

/// Validation result type containing validation status, message, and type information
#[derive(Debug, Clone, PartialEq)]
pub struct ValidateCoreReturnType<T> {
    pub validate: bool,
    pub message: String,
    pub type_value: T,
}

/// Validation return type for validators
pub struct ValidateReturnType<T> {
    pub type_name: String,
    pub message: Option<String>,
    pub validate: Arc<dyn Fn(&T) -> bool + Send + Sync>,
}

impl<T> ValidateReturnType<T> {
    pub fn new<F>(type_name: &str, message: Option<String>, validate: F) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        Self {
            type_name: type_name.to_string(),
            message,
            validate: Arc::new(validate),
        }
    }
}

impl<T> Clone for ValidateReturnType<T> {
    fn clone(&self) -> Self {
        Self {
            type_name: self.type_name.clone(),
            message: self.message.clone(),
            validate: Arc::clone(&self.validate),
        }
    }
}

impl<T> Debug for ValidateReturnType<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidateReturnType")
            .field("type_name", &self.type_name)
            .field("message", &self.message)
            .field("validate", &"<function>")
            .finish()
    }
}

/// Type name enum for validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeName {
    String,
    Number,
    Boolean,
    Undefined,
    Null,
    Object,
    Array,
}

impl Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeName::String => write!(f, "string"),
            TypeName::Number => write!(f, "number"),
            TypeName::Boolean => write!(f, "boolean"),
            TypeName::Undefined => write!(f, "undefined"),
            TypeName::Null => write!(f, "null"),
            TypeName::Object => write!(f, "object"),
            TypeName::Array => write!(f, "array"),
        }
    }
}

/// Core validation function that checks type and additional validation rules
///
/// # Arguments
/// * `value` - The value to validate
/// * `type_name` - The expected type name
/// * `options` - Array of validation functions to apply
/// * `message` - Custom error message for type validation
///
/// # Returns
/// A `ValidateCoreReturnType` containing the validation result
#[inline]
pub fn umt_validate_core<T: Clone>(
    value: T,
    _type_name: &str,
    options: &[ValidateReturnType<T>],
    message: Option<&str>,
) -> ValidateCoreReturnType<T> {
    for option in options {
        if !(option.validate)(&value) {
            return ValidateCoreReturnType {
                validate: false,
                message: option.message.clone().unwrap_or_default(),
                type_value: value,
            };
        }
    }

    ValidateCoreReturnType {
        validate: true,
        message: message.unwrap_or("").to_string(),
        type_value: value,
    }
}
