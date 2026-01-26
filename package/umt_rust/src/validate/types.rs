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
