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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_formatters() -> HashMap<String, Formatter> {
        let mut formatters: HashMap<String, Formatter> = HashMap::new();

        formatters.insert("upper".to_string(), Box::new(|v, _| v.to_uppercase()));
        formatters.insert("lower".to_string(), Box::new(|v, _| v.to_lowercase()));
        formatters.insert(
            "pad".to_string(),
            Box::new(|v, args| {
                let length: usize = args.get(0).and_then(|s| s.parse().ok()).unwrap_or(2);
                let pad_char = args
                    .get(1)
                    .map(|s| s.chars().next().unwrap_or('0'))
                    .unwrap_or('0');
                format!("{:>width$}", v, width = length).replace(' ', &pad_char.to_string())
            }),
        );
        formatters.insert(
            "multiply".to_string(),
            Box::new(|v, args| {
                let num: f64 = v.parse().unwrap_or(0.0);
                let factor: f64 = args.get(0).and_then(|s| s.parse().ok()).unwrap_or(1.0);
                (num * factor).to_string()
            }),
        );

        formatters
    }

    #[test]
    fn test_uppercase_formatter() {
        let formatters = create_mock_formatters();
        assert_eq!(apply_formatter("hello", "upper", &formatters), "HELLO");
        assert_eq!(apply_formatter("WORLD", "upper", &formatters), "WORLD");
        assert_eq!(apply_formatter("123", "upper", &formatters), "123");
    }

    #[test]
    fn test_lowercase_formatter() {
        let formatters = create_mock_formatters();
        assert_eq!(apply_formatter("HELLO", "lower", &formatters), "hello");
        assert_eq!(apply_formatter("WoRlD", "lower", &formatters), "world");
    }

    #[test]
    fn test_formatter_with_single_argument() {
        let formatters = create_mock_formatters();
        assert_eq!(apply_formatter("42", "multiply(2)", &formatters), "84");
    }

    #[test]
    fn test_invalid_formatter_syntax() {
        let formatters = create_mock_formatters();
        assert_eq!(apply_formatter("test", "invalid!@#", &formatters), "test");
        assert_eq!(apply_formatter("hello", "upper(", &formatters), "hello");
        assert_eq!(apply_formatter("world", "lower)", &formatters), "world");
    }

    #[test]
    fn test_non_existent_formatter() {
        let formatters = create_mock_formatters();
        assert_eq!(apply_formatter("test", "nonexistent", &formatters), "test");
        assert_eq!(apply_formatter("456", "missing(arg)", &formatters), "456");
    }

    #[test]
    fn test_empty_arguments() {
        let formatters = create_mock_formatters();
        assert_eq!(apply_formatter("test", "upper()", &formatters), "TEST");
    }

    #[test]
    fn test_empty_formatters() {
        let formatters: HashMap<String, Formatter> = HashMap::new();
        assert_eq!(apply_formatter("test", "upper", &formatters), "test");
    }
}
