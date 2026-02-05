pub mod apply_formatter;
pub use apply_formatter::*;

pub mod default_formatters;
pub use default_formatters::*;

pub mod get_value;
pub use get_value::*;

pub mod detect_mode;
pub use detect_mode::*;

use crate::object::Value;
use regex::Regex;
use std::collections::HashMap;

/// Formats a string by replacing placeholders with values.
///
/// # Arguments
///
/// * `template` - The template string
/// * `args` - Arguments for formatting
///
/// # Returns
///
/// Formatted string
pub fn umt_format_string(template: &str, args: &[Value]) -> String {
    if args.is_empty() {
        return template.to_string();
    }

    // Convert args to detect_mode input format
    let data_or_first = args.get(0).cloned();
    let options_or_second = args.get(1).cloned();
    let rest = if args.len() > 2 {
        args[2..].to_vec()
    } else {
        Vec::new()
    };

    let result = umt_detect_mode(data_or_first, options_or_second, rest);
    let data = result.data;
    let options = result.options;

    format_string_internal(template, &data, &options)
}

/// Helper function to format string with indexed args directly (for backward compatibility tests if any)
pub fn umt_format_string_indexed(template: &str, args: &[Value]) -> String {
     let data = Value::Array(args.to_vec());
     let options = HashMap::new();
     format_string_internal(template, &data, &options)
}

fn format_string_internal(
    template: &str,
    data: &Value,
    options: &HashMap<String, Value>,
) -> String {
    // Handle escaped braces
    let escaped_template = template.replace("{{", "\u{0000}").replace("}}", "\u{0001}");

    // Merge default formatters with custom formatters
    let formatters = create_default_formatters();
    if let Some(Value::Object(_custom_formatters)) = options.get("formatters") {
        // Since Value enum can't hold closures easily, we might not support custom formatters here
        // fully unless we change Value definition. For now, we only support default formatters.
        // If the custom formatters map contains keys that we can support (e.g. referencing other formatters?),
        // we could implement it. But TS allows passing functions.
        // For strict parity, we'd need a Value variant that holds a function, which is hard in Rust (serialization etc).
        // We will skip merging custom formatters for now and rely on defaults.
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
            // Indexed mode: path should be an index
            if let Ok(idx) = path.parse::<usize>() {
                if let Value::Array(arr) = data {
                    arr.get(idx)
                } else {
                    None
                }
            } else if let Ok(neg_idx) = path.parse::<isize>() {
                // Support negative indices
                if neg_idx < 0 {
                    if let Value::Array(arr) = data {
                        let abs_idx = neg_idx.abs() as usize;
                        if abs_idx <= arr.len() {
                            arr.get(arr.len() - abs_idx)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                // Path might be a property access on an object inside array?
                // TS implementation for indexed mode usually expects simple index.
                // But if data is array, get_value can handle "0.name" style paths?
                // Let's rely on get_value if path is complex.
                get_value(data, path)
            }
        } else {
            // Named mode
            get_value(data, path)
        };

        let value = match value {
            Some(Value::Null) => {
                if let Some(default) = default_value {
                    Value::String(default.to_string())
                } else {
                    Value::Null
                }
            }
            Some(v) => v.clone(),
            None => {
                if let Some(default) = default_value {
                    Value::String(default.to_string())
                } else {
                    // If no value and no default, return the full match (keep placeholder)
                    return caps[0].to_string();
                }
            }
        };

        // Convert value to string
        let value_str = match &value {
            Value::String(s) => s.clone(),
            Value::Int(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Array(arr) => arr
                .iter()
                .map(|v| v.to_string()) // Use Display impl of Value
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
