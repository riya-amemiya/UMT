pub mod apply_formatter;
pub mod default_formatters;
pub mod detect_mode;
pub mod get_value;

pub use apply_formatter::{Formatter, apply_formatter};
pub use default_formatters::create_default_formatters;
pub use detect_mode::{DetectModeResult, detect_mode};
pub use get_value::get_value;

use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

/// Options for format string customization
#[derive(Default)]
pub struct FormatOptions {
    pub formatters: HashMap<String, Formatter>,
}

/// Replaces placeholders in a template string with specified values.
///
/// Supports two modes:
/// 1. **Indexed mode**: Use numbered placeholders like {0}, {1}, {2}...
/// 2. **Named mode**: Use named placeholders with an object like {name}, {age}...
///
/// ## Features
///
/// ### Nested Object Access
/// Access nested properties using dot notation: `{user.name}`, `{user.address.city}`
///
/// ### Array Access
/// Access array elements with brackets: `{items[0]}`, `{users[1].name}`
/// Supports negative indices: `{items[-1]}` (last element)
///
/// ### Default Values
/// Provide fallback values using pipe syntax: `{name|Unknown}`, `{age|N/A}`
///
/// ### Formatters
/// Apply formatters to values: `{name:upper}`, `{count:plural(item,items)}`
///
/// Built-in formatters:
/// - `upper` - Convert to uppercase
/// - `lower` - Convert to lowercase
/// - `plural(singular, plural)` - Pluralization
/// - `pad(length, char?)` - Pad string/number
///
/// ### Escape Sequences
/// Use double braces to escape: `{{name}}` renders as literal `{name}`
///
/// # Arguments
///
/// * `template` - Template string containing placeholders
/// * `data` - JSON value containing the data
/// * `options` - Optional format options with custom formatters
///
/// # Returns
///
/// String with placeholders replaced with formatted values
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_format_string;
/// use serde_json::json;
///
/// // Named mode
/// let result = umt_format_string(
///     "Hello, {name}! You are {age} years old.",
///     &json!({"name": "Alice", "age": 25}),
///     None
/// );
/// assert_eq!(result, "Hello, Alice! You are 25 years old.");
///
/// // With formatters
/// let result = umt_format_string(
///     "Name: {name:upper}",
///     &json!({"name": "alice"}),
///     None
/// );
/// assert_eq!(result, "Name: ALICE");
/// ```
pub fn umt_format_string(template: &str, data: &Value, options: Option<FormatOptions>) -> String {
    // Handle escaped braces
    let escaped_template = template.replace("{{", "\u{0000}").replace("}}", "\u{0001}");

    let options = options.unwrap_or_default();

    // Merge default formatters with custom formatters
    let mut formatters = create_default_formatters();
    for (key, value) in options.formatters {
        formatters.insert(key, value);
    }

    let placeholder_regex = Regex::new(r"\{([^}]+)\}").unwrap();

    let result = placeholder_regex.replace_all(&escaped_template, |caps: &regex::Captures| {
        let content = caps.get(1).map(|m| m.as_str()).unwrap_or("");

        // Parse content: path|default:formatter
        let (path_and_formatter, default_value) = if let Some(pipe_pos) = content.find('|') {
            let (left, right) = content.split_at(pipe_pos);
            (left.trim(), Some(right[1..].trim()))
        } else {
            (content.trim(), None)
        };

        // Parse formatter
        let (path, formatter_string) = if let Some(colon_pos) = path_and_formatter.find(':') {
            let (left, right) = path_and_formatter.split_at(colon_pos);
            (left.trim(), Some(right[1..].trim()))
        } else {
            (path_and_formatter, None)
        };

        // Get value from data
        let value = if data.is_array() {
            // Indexed mode
            path.parse::<usize>().ok().and_then(|idx| data.get(idx))
        } else {
            // Named mode
            get_value(data, path)
        };

        let value = match value {
            Some(v) if !v.is_null() => v.clone(),
            _ => {
                if let Some(default) = default_value {
                    Value::String(default.to_string())
                } else {
                    return caps[0].to_string();
                }
            }
        };

        // Convert value to string
        let value_str = match &value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Array(arr) => arr
                .iter()
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    _ => v.to_string(),
                })
                .collect::<Vec<_>>()
                .join(","),
            Value::Object(_) => "[object Object]".to_string(),
            Value::Null => "null".to_string(),
        };

        // Apply formatter if present
        if let Some(fmt_str) = formatter_string {
            apply_formatter(&value_str, fmt_str, &formatters)
        } else {
            value_str
        }
    });

    // Restore escaped braces
    result.replace('\u{0000}', "{").replace('\u{0001}', "}")
}

/// Replaces placeholders in a template string with indexed values.
///
/// # Arguments
///
/// * `template` - Template string containing placeholders like {0}, {1}
/// * `values` - Array of values to substitute
///
/// # Returns
///
/// String with placeholders replaced
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_format_string_indexed;
///
/// let result = umt_format_string_indexed("Hello, {0}! It's {1} today.", &["World", "sunny"]);
/// assert_eq!(result, "Hello, World! It's sunny today.");
/// ```
pub fn umt_format_string_indexed(template: &str, values: &[&str]) -> String {
    let array: Vec<Value> = values
        .iter()
        .map(|&s| Value::String(s.to_string()))
        .collect();
    umt_format_string(template, &Value::Array(array), None)
}
