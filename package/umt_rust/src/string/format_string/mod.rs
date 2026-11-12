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

fn parse_radix(digits: &str, radix: u32) -> f64 {
    if digits.starts_with('+') {
        return f64::NAN;
    }
    u64::from_str_radix(digits, radix).map_or(f64::NAN, |parsed| parsed as f64)
}

fn js_number(text: &str) -> f64 {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return 0.0;
    }

    for (lower, upper, radix) in [("0x", "0X", 16), ("0o", "0O", 8), ("0b", "0B", 2)] {
        if let Some(digits) = trimmed
            .strip_prefix(lower)
            .or_else(|| trimmed.strip_prefix(upper))
        {
            return parse_radix(digits, radix);
        }
    }

    let unsigned = trimmed.strip_prefix(['+', '-']).unwrap_or(trimmed);
    if unsigned == "Infinity" {
        return if trimmed.starts_with('-') {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
    }

    let is_decimal_literal = unsigned
        .bytes()
        .all(|b| b.is_ascii_digit() || matches!(b, b'.' | b'e' | b'E' | b'+' | b'-'))
        && unsigned.bytes().any(|b| b.is_ascii_digit());
    if is_decimal_literal {
        trimmed.parse().unwrap_or(f64::NAN)
    } else {
        f64::NAN
    }
}

fn js_number_string(number: &serde_json::Number) -> String {
    if number.is_i64() || number.is_u64() {
        return number.to_string();
    }

    let value = number.as_f64().unwrap_or(f64::NAN);
    let magnitude = value.abs();
    if magnitude == 0.0 {
        return "0".to_string();
    }
    if !(1e-6..1e21).contains(&magnitude) {
        let exponential = format!("{value:e}");
        return match exponential.split_once('e') {
            Some((mantissa, exponent)) if !exponent.starts_with('-') => {
                format!("{mantissa}e+{exponent}")
            }
            _ => exponential,
        };
    }
    value.to_string()
}

fn js_string(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Number(number) => js_number_string(number),
        Value::Bool(flag) => flag.to_string(),
        Value::Array(items) => items
            .iter()
            .map(|item| {
                if item.is_null() {
                    String::new()
                } else {
                    js_string(item)
                }
            })
            .collect::<Vec<_>>()
            .join(","),
        Value::Object(_) => "[object Object]".to_string(),
        Value::Null => "null".to_string(),
    }
}

fn indexed_value<'a>(items: &'a [Value], path: &str) -> Option<&'a Value> {
    let index = js_number(path);
    if index.fract() != 0.0 || index < 0.0 {
        return None;
    }
    items.get(index as usize)
}

fn array_length(data: &Value, path: &str) -> Option<usize> {
    match path.rsplit_once('.') {
        Some((parent_path, "length")) => get_value(data, parent_path)?.as_array().map(Vec::len),
        _ => None,
    }
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
/// Array length is available as `{items.length}`
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
/// - `currency(locale?, currency?)` - Currency formatting
/// - `number(locale?, minFractionDigits?, maxFractionDigits?)` - Number formatting
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
        // Parse content: path:formatter|default
        let mut parts = caps[1].split('|').map(str::trim);
        let path_and_formatter = parts.next().unwrap_or("");
        let default_value = parts.next();

        // Parse formatter
        let (path, formatter_string) = path_and_formatter
            .split_once(':')
            .unwrap_or((path_and_formatter, ""));

        // Get value from data
        let value = match data {
            // Indexed mode
            Value::Array(items) => indexed_value(items, path),
            // Named mode
            _ => get_value(data, path),
        };

        let length = if value.is_none() && !data.is_array() {
            array_length(data, path)
        } else {
            None
        };

        // Convert value to string
        let value_str = match (value, length, default_value) {
            (Some(found), _, _) if !found.is_null() => js_string(found),
            (_, Some(length), _) => length.to_string(),
            (_, _, Some(default)) => default.to_string(),
            (_, _, None) => return caps[0].to_string(),
        };

        // Apply formatter if present
        if formatter_string.is_empty() {
            value_str
        } else {
            apply_formatter(&value_str, formatter_string, &formatters)
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
