//! Type definitions for the validation module
//!
//! This module provides core types used across all validation functions.

use std::collections::HashMap;

/// Represents different value types that can be validated
#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    String,
    Number,
    Boolean,
    Null,
    Undefined,
    Array,
    Object,
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::String => write!(f, "string"),
            ValueType::Number => write!(f, "number"),
            ValueType::Boolean => write!(f, "boolean"),
            ValueType::Null => write!(f, "null"),
            ValueType::Undefined => write!(f, "undefined"),
            ValueType::Array => write!(f, "array"),
            ValueType::Object => write!(f, "object"),
        }
    }
}

/// Core validation result type including validation status, message, and type information
#[derive(Debug, Clone)]
pub struct ValidateCoreResult<T> {
    pub validate: bool,
    pub message: String,
    pub value_type: ValueType,
    pub value: Option<T>,
}

impl<T> ValidateCoreResult<T> {
    /// Creates a successful validation result
    pub fn success(value: T, value_type: ValueType) -> Self {
        Self {
            validate: true,
            message: String::new(),
            value_type,
            value: Some(value),
        }
    }

    /// Creates a failed validation result
    pub fn failure(message: impl Into<String>, value_type: ValueType) -> Self {
        Self {
            validate: false,
            message: message.into(),
            value_type,
            value: None,
        }
    }

    /// Creates a failed validation result with an optional message
    pub fn failure_optional(message: Option<&str>, value_type: ValueType) -> Self {
        Self {
            validate: false,
            message: message.unwrap_or("").to_string(),
            value_type,
            value: None,
        }
    }
}

/// Type for validation functions that take a value and return a boolean
pub type ValidateFunction<T> = Box<dyn Fn(&T) -> bool>;

/// Extended validation result type including type information and validation function
pub struct ValidateReturnType<T> {
    pub value_type: ValueType,
    pub validate: Box<dyn Fn(&T) -> bool>,
    pub message: Option<String>,
}

impl<T> ValidateReturnType<T> {
    /// Creates a new validation return type
    pub fn new(
        value_type: ValueType,
        validate: impl Fn(&T) -> bool + 'static,
        message: Option<String>,
    ) -> Self {
        Self {
            value_type,
            validate: Box::new(validate),
            message,
        }
    }
}

/// Dynamic value type that can hold any JSON-like value
#[derive(Debug, Clone, PartialEq)]
pub enum DynamicValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<DynamicValue>),
    Object(HashMap<String, DynamicValue>),
}

impl DynamicValue {
    /// Returns the type of the value
    pub fn value_type(&self) -> ValueType {
        match self {
            DynamicValue::Null => ValueType::Null,
            DynamicValue::Bool(_) => ValueType::Boolean,
            DynamicValue::Integer(_) | DynamicValue::Float(_) => ValueType::Number,
            DynamicValue::String(_) => ValueType::String,
            DynamicValue::Array(_) => ValueType::Array,
            DynamicValue::Object(_) => ValueType::Object,
        }
    }

    /// Checks if the value is null
    pub fn is_null(&self) -> bool {
        matches!(self, DynamicValue::Null)
    }

    /// Checks if the value is a boolean
    pub fn is_bool(&self) -> bool {
        matches!(self, DynamicValue::Bool(_))
    }

    /// Checks if the value is a number (integer or float)
    pub fn is_number(&self) -> bool {
        matches!(self, DynamicValue::Integer(_) | DynamicValue::Float(_))
    }

    /// Checks if the value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, DynamicValue::String(_))
    }

    /// Checks if the value is an array
    pub fn is_array(&self) -> bool {
        matches!(self, DynamicValue::Array(_))
    }

    /// Checks if the value is an object
    pub fn is_object(&self) -> bool {
        matches!(self, DynamicValue::Object(_))
    }

    /// Gets the value as a boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DynamicValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Gets the value as an i64
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            DynamicValue::Integer(n) => Some(*n),
            DynamicValue::Float(f) => Some(*f as i64),
            _ => None,
        }
    }

    /// Gets the value as an f64
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DynamicValue::Integer(n) => Some(*n as f64),
            DynamicValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Gets the value as a string reference
    pub fn as_str(&self) -> Option<&str> {
        match self {
            DynamicValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Gets the value as an array reference
    pub fn as_array(&self) -> Option<&Vec<DynamicValue>> {
        match self {
            DynamicValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Gets the value as an object reference
    pub fn as_object(&self) -> Option<&HashMap<String, DynamicValue>> {
        match self {
            DynamicValue::Object(obj) => Some(obj),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_type_display() {
        assert_eq!(ValueType::String.to_string(), "string");
        assert_eq!(ValueType::Number.to_string(), "number");
        assert_eq!(ValueType::Boolean.to_string(), "boolean");
        assert_eq!(ValueType::Null.to_string(), "null");
        assert_eq!(ValueType::Undefined.to_string(), "undefined");
        assert_eq!(ValueType::Array.to_string(), "array");
        assert_eq!(ValueType::Object.to_string(), "object");
    }

    #[test]
    fn test_validate_core_result_success() {
        let result = ValidateCoreResult::success(42, ValueType::Number);
        assert!(result.validate);
        assert_eq!(result.message, "");
        assert_eq!(result.value_type, ValueType::Number);
        assert_eq!(result.value, Some(42));
    }

    #[test]
    fn test_validate_core_result_failure() {
        let result: ValidateCoreResult<i32> =
            ValidateCoreResult::failure("Invalid value", ValueType::Number);
        assert!(!result.validate);
        assert_eq!(result.message, "Invalid value");
        assert_eq!(result.value_type, ValueType::Number);
        assert_eq!(result.value, None);
    }

    #[test]
    fn test_dynamic_value_type_detection() {
        assert_eq!(DynamicValue::Null.value_type(), ValueType::Null);
        assert_eq!(DynamicValue::Bool(true).value_type(), ValueType::Boolean);
        assert_eq!(DynamicValue::Integer(42).value_type(), ValueType::Number);
        assert_eq!(DynamicValue::Float(3.14).value_type(), ValueType::Number);
        assert_eq!(
            DynamicValue::String("test".to_string()).value_type(),
            ValueType::String
        );
        assert_eq!(DynamicValue::Array(vec![]).value_type(), ValueType::Array);
        assert_eq!(
            DynamicValue::Object(HashMap::new()).value_type(),
            ValueType::Object
        );
    }

    #[test]
    fn test_dynamic_value_type_checks() {
        let null = DynamicValue::Null;
        assert!(null.is_null());
        assert!(!null.is_bool());

        let bool_val = DynamicValue::Bool(true);
        assert!(bool_val.is_bool());
        assert!(!bool_val.is_number());

        let int_val = DynamicValue::Integer(42);
        assert!(int_val.is_number());
        assert!(!int_val.is_string());

        let float_val = DynamicValue::Float(3.14);
        assert!(float_val.is_number());

        let string_val = DynamicValue::String("test".to_string());
        assert!(string_val.is_string());
        assert!(!string_val.is_array());

        let array_val = DynamicValue::Array(vec![]);
        assert!(array_val.is_array());
        assert!(!array_val.is_object());

        let object_val = DynamicValue::Object(HashMap::new());
        assert!(object_val.is_object());
        assert!(!object_val.is_null());
    }

    #[test]
    fn test_dynamic_value_conversions() {
        assert_eq!(DynamicValue::Bool(true).as_bool(), Some(true));
        assert_eq!(DynamicValue::Integer(42).as_i64(), Some(42));
        assert_eq!(DynamicValue::Float(3.14).as_f64(), Some(3.14));
        assert_eq!(
            DynamicValue::String("test".to_string()).as_str(),
            Some("test")
        );

        let arr = vec![DynamicValue::Integer(1), DynamicValue::Integer(2)];
        let array_val = DynamicValue::Array(arr.clone());
        assert_eq!(array_val.as_array(), Some(&arr));

        let mut obj = HashMap::new();
        obj.insert("key".to_string(), DynamicValue::Integer(42));
        let object_val = DynamicValue::Object(obj.clone());
        assert_eq!(object_val.as_object(), Some(&obj));
    }
}
