use regex::Regex;
use std::collections::HashMap;

/// A formatter function type
pub type Formatter = Box<dyn Fn(&str, &[String]) -> String + Send + Sync>;

/// Applies a formatter function to a value with optional arguments.
///
/// Parses formatter syntax like "upper", "currency(ja-JP,JPY)", "pad(4,0)" and applies
/// the corresponding formatter function with parsed arguments.
///
/// # Arguments
///
/// * `value` - The value to format (as string)
/// * `formatter_string` - Formatter name with optional arguments (e.g., "upper", "pad(4,0)")
/// * `formatters` - Available formatter functions
///
/// # Returns
///
/// Formatted string, or original string value if formatter not found/invalid
pub fn apply_formatter(
    value: &str,
    formatter_string: &str,
    formatters: &HashMap<String, Formatter>,
) -> String {
    let pattern = Regex::new(r"^(\w+)(?:\(([^)]*)\))?$").unwrap();

    let Some(caps) = pattern.captures(formatter_string) else {
        return value.to_string();
    };

    let formatter_name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
    let arguments_string = caps.get(2).map(|m| m.as_str());

    let Some(formatter) = formatters.get(formatter_name) else {
        return value.to_string();
    };

    let arguments = arguments_string.map(parse_arguments).unwrap_or_default();

    formatter(value, &arguments)
}

/// Parses comma-separated arguments while preserving quoted strings
fn parse_arguments(arguments_string: &str) -> Vec<String> {
    let mut arguments = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';

    for c in arguments_string.chars() {
        if !in_quotes && (c == '"' || c == '\'') {
            in_quotes = true;
            quote_char = c;
            continue;
        }

        if in_quotes && c == quote_char {
            in_quotes = false;
            quote_char = ' ';
            continue;
        }

        if !in_quotes && c == ',' {
            let trimmed = current.trim();
            arguments.push(if trimmed.is_empty() {
                " ".to_string()
            } else {
                trimmed.to_string()
            });
            current.clear();
            continue;
        }

        current.push(c);
    }

    // Handle last argument
    let trimmed = current.trim();
    arguments.push(if trimmed.is_empty() {
        " ".to_string()
    } else {
        trimmed.to_string()
    });

    arguments
}
