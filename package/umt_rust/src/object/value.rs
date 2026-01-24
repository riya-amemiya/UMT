use std::collections::HashMap;
use std::fmt;

/// A dynamic value type for representing JSON-like structures.
/// This allows working with heterogeneous data structures similar to JavaScript objects.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Null value
    Null,
    /// Boolean value
    Bool(bool),
    /// Integer value
    Int(i64),
    /// Floating point value
    Float(f64),
    /// String value
    String(String),
    /// Array of values
    Array(Vec<Value>),
    /// Object (map of string keys to values)
    Object(HashMap<String, Value>),
}

impl Value {
    /// Returns true if this value is an object.
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    /// Returns true if this value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns the value as an object if it is one.
    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(map) => Some(map),
            _ => None,
        }
    }

    /// Returns the value as a mutable object if it is one.
    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        match self {
            Value::Object(map) => Some(map),
            _ => None,
        }
    }

    /// Converts the value into an object if it is one.
    pub fn into_object(self) -> Option<HashMap<String, Value>> {
        match self {
            Value::Object(map) => Some(map),
            _ => None,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Object(obj) => {
                write!(f, "{{")?;
                for (i, (k, v)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

// Conversion traits for convenient Value creation
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Int(i as i64)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(v: Vec<T>) -> Self {
        Value::Array(v.into_iter().map(|x| x.into()).collect())
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(m: HashMap<String, Value>) -> Self {
        Value::Object(m)
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(v) => v.into(),
            None => Value::Null,
        }
    }
}

/// Macro for creating a Value::Object easily
#[macro_export]
macro_rules! obj {
    () => {
        $crate::object::Value::Object(std::collections::HashMap::new())
    };
    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key.to_string(), $crate::object::Value::from($value));
        )+
        $crate::object::Value::Object(map)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_creation() {
        let null = Value::Null;
        assert!(null.is_null());

        let bool_val = Value::Bool(true);
        assert!(!bool_val.is_object());

        let int_val = Value::Int(42);
        assert_eq!(int_val, Value::Int(42));

        let float_val = Value::Float(3.14);
        assert_eq!(float_val, Value::Float(3.14));

        let string_val = Value::String("hello".to_string());
        assert_eq!(string_val, Value::String("hello".to_string()));
    }

    #[test]
    fn test_value_object() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::Int(123));
        let obj = Value::Object(map);

        assert!(obj.is_object());
        assert!(obj.as_object().is_some());
    }

    #[test]
    fn test_value_conversions() {
        let bool_val: Value = true.into();
        assert_eq!(bool_val, Value::Bool(true));

        let int_val: Value = 42i32.into();
        assert_eq!(int_val, Value::Int(42));

        let string_val: Value = "hello".into();
        assert_eq!(string_val, Value::String("hello".to_string()));
    }
}
