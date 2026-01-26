use serde::de::DeserializeOwned;
use serde_json;

/// Error type for JSON parsing failures
#[derive(Debug, Clone, PartialEq)]
pub struct JsonParseError {
    pub message: String,
}

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for JsonParseError {}

impl From<serde_json::Error> for JsonParseError {
    fn from(err: serde_json::Error) -> Self {
        JsonParseError {
            message: err.to_string(),
        }
    }
}

/// Parses a JSON string into a typed Rust value.
///
/// # Arguments
///
/// * `json` - A JSON string to parse
///
/// # Returns
///
/// Returns `Ok(T)` with the parsed value on success,
/// or `Err(JsonParseError)` if parsing fails.
///
/// # Example
///
/// ```
/// use umt_rust::tool::umt_parse_json;
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug, PartialEq)]
/// struct Person {
///     name: String,
///     age: i32,
/// }
///
/// let json = r#"{"name": "John", "age": 30}"#;
/// let person: Person = umt_parse_json(json).unwrap();
/// assert_eq!(person.name, "John");
/// assert_eq!(person.age, 30);
/// ```
#[inline]
pub fn umt_parse_json<T: DeserializeOwned>(json: &str) -> Result<T, JsonParseError> {
    serde_json::from_str(json).map_err(JsonParseError::from)
}

/// Parses a JSON string into a serde_json::Value.
///
/// This is useful when you don't know the structure of the JSON beforehand.
///
/// # Arguments
///
/// * `json` - A JSON string to parse
///
/// # Returns
///
/// Returns `Ok(serde_json::Value)` on success,
/// or `Err(JsonParseError)` if parsing fails.
///
/// # Example
///
/// ```
/// use umt_rust::tool::umt_parse_json_value;
///
/// let json = r#"{"key": "value"}"#;
/// let value = umt_parse_json_value(json).unwrap();
/// assert_eq!(value["key"], "value");
/// ```
#[inline]
pub fn umt_parse_json_value(json: &str) -> Result<serde_json::Value, JsonParseError> {
    umt_parse_json(json)
}

