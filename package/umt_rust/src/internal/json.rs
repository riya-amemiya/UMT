use std::collections::BTreeMap;
use std::fmt;

// ---- JsonValue type ----

/// A JSON value type that replaces serde_json::Value.
#[derive(Debug, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

impl JsonValue {
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, JsonValue::Bool(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, JsonValue::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, JsonValue::String(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, JsonValue::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, JsonValue::Object(_))
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            JsonValue::Number(n) => {
                if *n >= i64::MIN as f64 && *n <= i64::MAX as f64 && *n == (*n as i64) as f64 {
                    Some(*n as i64)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&BTreeMap<String, JsonValue>> {
        match self {
            JsonValue::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Get a value by string key (for objects) or by index (for arrays by string index)
    pub fn get<I: JsonIndex>(&self, index: I) -> Option<&JsonValue> {
        index.index_into(self)
    }

    /// Convert to string representation for display
    pub fn to_json_string(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => format_number(*n),
            JsonValue::String(s) => format!("\"{}\"", escape_json_string(s)),
            JsonValue::Array(a) => {
                let items: Vec<String> = a.iter().map(|v| v.to_json_string()).collect();
                format!("[{}]", items.join(","))
            }
            JsonValue::Object(o) => {
                let items: Vec<String> = o
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", escape_json_string(k), v.to_json_string()))
                    .collect();
                format!("{{{}}}", items.join(","))
            }
        }
    }
}

fn format_number(n: f64) -> String {
    if n == (n as i64) as f64 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{}", n)
    }
}

fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}

// ---- Index trait ----

pub trait JsonIndex {
    fn index_into<'a>(&self, value: &'a JsonValue) -> Option<&'a JsonValue>;
}

impl JsonIndex for &str {
    fn index_into<'a>(&self, value: &'a JsonValue) -> Option<&'a JsonValue> {
        match value {
            JsonValue::Object(map) => map.get(*self),
            _ => None,
        }
    }
}

impl JsonIndex for String {
    fn index_into<'a>(&self, value: &'a JsonValue) -> Option<&'a JsonValue> {
        match value {
            JsonValue::Object(map) => map.get(self.as_str()),
            _ => None,
        }
    }
}

impl JsonIndex for usize {
    fn index_into<'a>(&self, value: &'a JsonValue) -> Option<&'a JsonValue> {
        match value {
            JsonValue::Array(arr) => arr.get(*self),
            _ => None,
        }
    }
}

// ---- Index operator ----

impl std::ops::Index<&str> for JsonValue {
    type Output = JsonValue;
    fn index(&self, key: &str) -> &JsonValue {
        static NULL: JsonValue = JsonValue::Null;
        self.get(key).unwrap_or(&NULL)
    }
}

impl std::ops::Index<usize> for JsonValue {
    type Output = JsonValue;
    fn index(&self, idx: usize) -> &JsonValue {
        static NULL: JsonValue = JsonValue::Null;
        self.get(idx).unwrap_or(&NULL)
    }
}

// ---- PartialEq implementations ----

impl PartialEq for JsonValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (JsonValue::Null, JsonValue::Null) => true,
            (JsonValue::Bool(a), JsonValue::Bool(b)) => a == b,
            (JsonValue::Number(a), JsonValue::Number(b)) => a == b,
            (JsonValue::String(a), JsonValue::String(b)) => a == b,
            (JsonValue::Array(a), JsonValue::Array(b)) => a == b,
            (JsonValue::Object(a), JsonValue::Object(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq<&str> for JsonValue {
    fn eq(&self, other: &&str) -> bool {
        match self {
            JsonValue::String(s) => s.as_str() == *other,
            _ => false,
        }
    }
}

impl PartialEq<bool> for JsonValue {
    fn eq(&self, other: &bool) -> bool {
        match self {
            JsonValue::Bool(b) => b == other,
            _ => false,
        }
    }
}

impl PartialEq<i32> for JsonValue {
    fn eq(&self, other: &i32) -> bool {
        match self {
            JsonValue::Number(n) => *n == *other as f64,
            _ => false,
        }
    }
}

impl PartialEq<i64> for JsonValue {
    fn eq(&self, other: &i64) -> bool {
        match self {
            JsonValue::Number(n) => *n == *other as f64,
            _ => false,
        }
    }
}

impl PartialEq<f64> for JsonValue {
    fn eq(&self, other: &f64) -> bool {
        match self {
            JsonValue::Number(n) => n == other,
            _ => false,
        }
    }
}

// ---- From implementations ----

impl From<()> for JsonValue {
    fn from(_: ()) -> Self {
        JsonValue::Null
    }
}

impl From<bool> for JsonValue {
    fn from(b: bool) -> Self {
        JsonValue::Bool(b)
    }
}

impl From<i32> for JsonValue {
    fn from(n: i32) -> Self {
        JsonValue::Number(n as f64)
    }
}

impl From<i64> for JsonValue {
    fn from(n: i64) -> Self {
        JsonValue::Number(n as f64)
    }
}

impl From<u32> for JsonValue {
    fn from(n: u32) -> Self {
        JsonValue::Number(n as f64)
    }
}

impl From<u64> for JsonValue {
    fn from(n: u64) -> Self {
        JsonValue::Number(n as f64)
    }
}

impl From<f64> for JsonValue {
    fn from(n: f64) -> Self {
        JsonValue::Number(n)
    }
}

impl From<&str> for JsonValue {
    fn from(s: &str) -> Self {
        JsonValue::String(s.to_string())
    }
}

impl From<String> for JsonValue {
    fn from(s: String) -> Self {
        JsonValue::String(s)
    }
}

impl<T: Into<JsonValue>> From<Vec<T>> for JsonValue {
    fn from(v: Vec<T>) -> Self {
        JsonValue::Array(v.into_iter().map(|x| x.into()).collect())
    }
}

impl<T: Into<JsonValue>> From<Option<T>> for JsonValue {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(v) => v.into(),
            None => JsonValue::Null,
        }
    }
}

// ---- Display ----

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => write!(f, "{}", format_number(*n)),
            JsonValue::String(s) => write!(f, "{}", s),
            JsonValue::Array(_) | JsonValue::Object(_) => write!(f, "{}", self.to_json_string()),
        }
    }
}

// ---- JSON Parser ----

#[derive(Debug, Clone, PartialEq)]
pub struct JsonParseError {
    pub message: String,
}

impl fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for JsonParseError {}

pub fn parse_json(input: &str) -> Result<JsonValue, JsonParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(JsonParseError {
            message: "Empty input".to_string(),
        });
    }

    let bytes = input.as_bytes();
    let (value, pos) = parse_value(bytes, 0)?;

    // Check for trailing content
    let pos = skip_whitespace(bytes, pos);
    if pos != bytes.len() {
        return Err(JsonParseError {
            message: format!("Unexpected trailing content at position {}", pos),
        });
    }

    Ok(value)
}

fn skip_whitespace(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && matches!(bytes[pos], b' ' | b'\t' | b'\n' | b'\r') {
        pos += 1;
    }
    pos
}

fn parse_value(bytes: &[u8], pos: usize) -> Result<(JsonValue, usize), JsonParseError> {
    let pos = skip_whitespace(bytes, pos);
    if pos >= bytes.len() {
        return Err(JsonParseError {
            message: "Unexpected end of input".to_string(),
        });
    }

    match bytes[pos] {
        b'n' => parse_null(bytes, pos),
        b't' | b'f' => parse_bool(bytes, pos),
        b'"' => parse_string(bytes, pos),
        b'[' => parse_array(bytes, pos),
        b'{' => parse_object(bytes, pos),
        b'-' | b'0'..=b'9' => parse_number(bytes, pos),
        ch => Err(JsonParseError {
            message: format!("Unexpected character '{}' at position {}", ch as char, pos),
        }),
    }
}

fn parse_null(bytes: &[u8], pos: usize) -> Result<(JsonValue, usize), JsonParseError> {
    if bytes.len() >= pos + 4 && &bytes[pos..pos + 4] == b"null" {
        Ok((JsonValue::Null, pos + 4))
    } else {
        Err(JsonParseError {
            message: format!("Invalid token at position {}", pos),
        })
    }
}

fn parse_bool(bytes: &[u8], pos: usize) -> Result<(JsonValue, usize), JsonParseError> {
    if bytes.len() >= pos + 4 && &bytes[pos..pos + 4] == b"true" {
        Ok((JsonValue::Bool(true), pos + 4))
    } else if bytes.len() >= pos + 5 && &bytes[pos..pos + 5] == b"false" {
        Ok((JsonValue::Bool(false), pos + 5))
    } else {
        Err(JsonParseError {
            message: format!("Invalid token at position {}", pos),
        })
    }
}

fn parse_string(bytes: &[u8], pos: usize) -> Result<(JsonValue, usize), JsonParseError> {
    let (s, new_pos) = parse_string_raw(bytes, pos)?;
    Ok((JsonValue::String(s), new_pos))
}

fn parse_string_raw(bytes: &[u8], pos: usize) -> Result<(String, usize), JsonParseError> {
    if bytes[pos] != b'"' {
        return Err(JsonParseError {
            message: format!("Expected '\"' at position {}", pos),
        });
    }

    let mut result = String::new();
    let mut i = pos + 1;

    while i < bytes.len() {
        match bytes[i] {
            b'"' => return Ok((result, i + 1)),
            b'\\' => {
                i += 1;
                if i >= bytes.len() {
                    return Err(JsonParseError {
                        message: "Unexpected end of string".to_string(),
                    });
                }
                match bytes[i] {
                    b'"' => result.push('"'),
                    b'\\' => result.push('\\'),
                    b'/' => result.push('/'),
                    b'b' => result.push('\u{0008}'),
                    b'f' => result.push('\u{000C}'),
                    b'n' => result.push('\n'),
                    b'r' => result.push('\r'),
                    b't' => result.push('\t'),
                    b'u' => {
                        if i + 4 >= bytes.len() {
                            return Err(JsonParseError {
                                message: "Invalid unicode escape".to_string(),
                            });
                        }
                        let hex = std::str::from_utf8(&bytes[i + 1..i + 5]).map_err(|_| {
                            JsonParseError {
                                message: "Invalid unicode escape".to_string(),
                            }
                        })?;
                        let code_point =
                            u32::from_str_radix(hex, 16).map_err(|_| JsonParseError {
                                message: "Invalid unicode escape".to_string(),
                            })?;
                        if let Some(c) = char::from_u32(code_point) {
                            result.push(c);
                        }
                        i += 4;
                    }
                    _ => {
                        result.push('\\');
                        result.push(bytes[i] as char);
                    }
                }
            }
            b => {
                // Handle UTF-8 properly
                if b < 0x80 {
                    result.push(b as char);
                } else {
                    // Read full UTF-8 character
                    let remaining = &bytes[i..];
                    if let Ok(s) = std::str::from_utf8(remaining) {
                        if let Some(ch) = s.chars().next() {
                            result.push(ch);
                            i += ch.len_utf8() - 1;
                        }
                    }
                }
            }
        }
        i += 1;
    }

    Err(JsonParseError {
        message: "Unterminated string".to_string(),
    })
}

fn parse_number(bytes: &[u8], pos: usize) -> Result<(JsonValue, usize), JsonParseError> {
    let mut i = pos;

    // Optional negative sign
    if i < bytes.len() && bytes[i] == b'-' {
        i += 1;
    }

    // Integer part
    if i < bytes.len() && bytes[i] == b'0' {
        i += 1;
    } else {
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }

    // Fractional part
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }

    // Exponent part
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        i += 1;
        if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
            i += 1;
        }
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }

    let num_str = std::str::from_utf8(&bytes[pos..i]).map_err(|_| JsonParseError {
        message: format!("Invalid number at position {}", pos),
    })?;

    let n: f64 = num_str.parse().map_err(|_| JsonParseError {
        message: format!("Invalid number '{}' at position {}", num_str, pos),
    })?;

    Ok((JsonValue::Number(n), i))
}

fn parse_array(bytes: &[u8], pos: usize) -> Result<(JsonValue, usize), JsonParseError> {
    let mut i = pos + 1; // skip '['
    let mut arr = Vec::new();

    i = skip_whitespace(bytes, i);
    if i < bytes.len() && bytes[i] == b']' {
        return Ok((JsonValue::Array(arr), i + 1));
    }

    loop {
        let (value, new_pos) = parse_value(bytes, i)?;
        arr.push(value);
        i = skip_whitespace(bytes, new_pos);

        if i >= bytes.len() {
            return Err(JsonParseError {
                message: "Unterminated array".to_string(),
            });
        }

        match bytes[i] {
            b',' => i += 1,
            b']' => return Ok((JsonValue::Array(arr), i + 1)),
            _ => {
                return Err(JsonParseError {
                    message: format!("Expected ',' or ']' at position {}", i),
                })
            }
        }
    }
}

fn parse_object(bytes: &[u8], pos: usize) -> Result<(JsonValue, usize), JsonParseError> {
    let mut i = pos + 1; // skip '{'
    let mut map = BTreeMap::new();

    i = skip_whitespace(bytes, i);
    if i < bytes.len() && bytes[i] == b'}' {
        return Ok((JsonValue::Object(map), i + 1));
    }

    loop {
        i = skip_whitespace(bytes, i);
        if i >= bytes.len() || bytes[i] != b'"' {
            return Err(JsonParseError {
                message: format!("Expected string key at position {}", i),
            });
        }

        let (key, new_pos) = parse_string_raw(bytes, i)?;
        i = skip_whitespace(bytes, new_pos);

        if i >= bytes.len() || bytes[i] != b':' {
            return Err(JsonParseError {
                message: format!("Expected ':' at position {}", i),
            });
        }
        i += 1;

        let (value, new_pos) = parse_value(bytes, i)?;
        map.insert(key, value);
        i = skip_whitespace(bytes, new_pos);

        if i >= bytes.len() {
            return Err(JsonParseError {
                message: "Unterminated object".to_string(),
            });
        }

        match bytes[i] {
            b',' => i += 1,
            b'}' => return Ok((JsonValue::Object(map), i + 1)),
            _ => {
                return Err(JsonParseError {
                    message: format!("Expected ',' or '}}' at position {}", i),
                })
            }
        }
    }
}

// ---- json! macro ----

/// Creates a `JsonValue` from a JSON-like literal.
///
/// # Examples
/// ```
/// use umt_rust::json;
///
/// let val = json!(null);
/// let val = json!(true);
/// let val = json!(42);
/// let val = json!("hello");
/// let val = json!([1, 2, 3]);
/// let val = json!({"key": "value"});
/// ```
#[macro_export]
macro_rules! json {
    (null) => {
        $crate::internal::json::JsonValue::Null
    };
    (true) => {
        $crate::internal::json::JsonValue::Bool(true)
    };
    (false) => {
        $crate::internal::json::JsonValue::Bool(false)
    };
    ([]) => {
        $crate::internal::json::JsonValue::Array(vec![])
    };
    ([ $($elem:tt),* $(,)? ]) => {
        $crate::internal::json::JsonValue::Array(vec![ $( $crate::json!($elem) ),* ])
    };
    ({}) => {
        $crate::internal::json::JsonValue::Object(std::collections::BTreeMap::new())
    };
    ({ $($key:tt : $value:tt),* $(,)? }) => {
        {
            #[allow(unused_mut)]
            let mut map = std::collections::BTreeMap::new();
            $( map.insert(($key).to_string(), $crate::json!($value)); )*
            $crate::internal::json::JsonValue::Object(map)
        }
    };
    ($e:expr) => {
        $crate::internal::json::JsonValue::from($e)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_null() {
        assert_eq!(parse_json("null").unwrap(), JsonValue::Null);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_json("true").unwrap(), JsonValue::Bool(true));
        assert_eq!(parse_json("false").unwrap(), JsonValue::Bool(false));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_json("42").unwrap(), JsonValue::Number(42.0));
        assert_eq!(parse_json("-3.14").unwrap(), JsonValue::Number(-3.14));
        assert_eq!(parse_json("0").unwrap(), JsonValue::Number(0.0));
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(
            parse_json("\"hello\"").unwrap(),
            JsonValue::String("hello".to_string())
        );
        assert_eq!(
            parse_json("\"hello\\nworld\"").unwrap(),
            JsonValue::String("hello\nworld".to_string())
        );
    }

    #[test]
    fn test_parse_array() {
        let result = parse_json("[1, 2, 3]").unwrap();
        if let JsonValue::Array(arr) = &result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], JsonValue::Number(1.0));
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_parse_object() {
        let result = parse_json("{\"key\": \"value\"}").unwrap();
        assert_eq!(result["key"], "value");
    }

    #[test]
    fn test_parse_nested() {
        let result = parse_json("{\"a\": {\"b\": [1, 2]}}").unwrap();
        assert_eq!(result["a"]["b"][0], 1);
        assert_eq!(result["a"]["b"][1], 2);
    }

    #[test]
    fn test_json_macro() {
        let val = json!(null);
        assert!(val.is_null());

        let val = json!(true);
        assert_eq!(val, true);

        let val = json!(42);
        assert_eq!(val, 42);

        let val = json!("hello");
        assert_eq!(val, "hello");

        let val = json!([1, 2, 3]);
        assert!(val.is_array());

        let val = json!({"name": "Alice", "age": 30});
        assert_eq!(val["name"], "Alice");
        assert_eq!(val["age"], 30);
    }

    #[test]
    fn test_json_macro_nested() {
        let val = json!({
            "user": {
                "name": "Bob",
                "tags": ["admin", "user"]
            }
        });
        assert_eq!(val["user"]["name"], "Bob");
        assert_eq!(val["user"]["tags"][0], "admin");
    }

    #[test]
    fn test_index_missing() {
        let val = json!({"key": "value"});
        assert!(val["missing"].is_null());
    }

    #[test]
    fn test_parse_error() {
        assert!(parse_json("").is_err());
        assert!(parse_json("{invalid}").is_err());
        assert!(parse_json("{\"key\":}").is_err());
    }

    #[test]
    fn test_display() {
        let val = json!(42);
        assert_eq!(format!("{}", val), "42");

        let val = json!("hello");
        assert_eq!(format!("{}", val), "hello");

        let val = json!(null);
        assert_eq!(format!("{}", val), "null");
    }

    #[test]
    fn test_partial_eq_str() {
        let val = json!("test");
        assert_eq!(val, "test");
    }

    #[test]
    fn test_partial_eq_number() {
        let val = json!(42);
        assert_eq!(val, 42);
        assert_eq!(val, 42.0_f64);
    }

    #[test]
    fn test_from_conversions() {
        assert!(matches!(JsonValue::from(42), JsonValue::Number(_)));
        assert!(matches!(JsonValue::from("hi"), JsonValue::String(_)));
        assert!(matches!(JsonValue::from(true), JsonValue::Bool(true)));
        assert!(matches!(JsonValue::from(3.14_f64), JsonValue::Number(_)));
    }
}
