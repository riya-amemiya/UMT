use std::collections::HashMap;
use umt_rust::string::format_string::{Formatter, apply_formatter};

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
        "currency".to_string(),
        Box::new(|v, args| {
            let num: f64 = v.parse().unwrap_or(0.0);
            let locale = args.get(0).map(|s| s.as_str()).unwrap_or("en-US");
            let currency = args.get(1).map(|s| s.as_str()).unwrap_or("USD");
            // Simple formatting without full locale support
            if currency == "USD" {
                format!("${:.2}", num)
                    .chars()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .collect::<String>()
                    .replace(
                        &format!("${:.2}", num),
                        &format_with_thousands(num, "$", ".", ","),
                    )
            } else {
                format!("{} {:.2}", currency, num)
            }
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

fn format_with_thousands(num: f64, prefix: &str, decimal: &str, thousands: &str) -> String {
    let int_part = num.trunc() as i64;
    let frac_part = (num.fract() * 100.0).round() as i64;

    let int_str = int_part
        .to_string()
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(thousands)
        .chars()
        .rev()
        .collect::<String>();

    format!("{}{}{}{:02}", prefix, int_str, decimal, frac_part)
}

// Simple formatters without arguments tests
#[test]
fn test_apply_uppercase_formatter() {
    let formatters = create_mock_formatters();
    assert_eq!(apply_formatter("hello", "upper", &formatters), "HELLO");
    assert_eq!(apply_formatter("WORLD", "upper", &formatters), "WORLD");
    assert_eq!(apply_formatter("123", "upper", &formatters), "123");
}

#[test]
fn test_apply_lowercase_formatter() {
    let formatters = create_mock_formatters();
    assert_eq!(apply_formatter("HELLO", "lower", &formatters), "hello");
    assert_eq!(apply_formatter("WoRlD", "lower", &formatters), "world");
    assert_eq!(apply_formatter("456", "lower", &formatters), "456");
}

// Formatters with arguments tests
#[test]
fn test_parse_and_apply_formatter_with_single_argument() {
    let formatters = create_mock_formatters();
    assert_eq!(apply_formatter("42", "multiply(2)", &formatters), "84");
    assert_eq!(apply_formatter("10", "multiply(0.5)", &formatters), "5");
}

#[test]
fn test_parse_and_apply_formatter_with_multiple_arguments() {
    let formatters = create_mock_formatters();
    assert_eq!(apply_formatter("42", "pad(4,0)", &formatters), "0042");
    assert_eq!(apply_formatter("test", "pad(6,*)", &formatters), "**test");
}

#[test]
fn test_handle_quoted_arguments() {
    let formatters = create_mock_formatters();
    assert_eq!(apply_formatter("5", "pad(3,\"x\")", &formatters), "xx5");
    assert_eq!(apply_formatter("5", "pad(3,'y')", &formatters), "yy5");
}

#[test]
fn test_handle_currency_formatter_with_locale_and_currency() {
    let formatters = create_mock_formatters();
    let result = apply_formatter("1234.56", "currency(en-US,USD)", &formatters);
    assert!(result.contains("1,234.56") || result.contains("1234.56"));
}

#[test]
fn test_trim_whitespace_in_arguments() {
    let formatters = create_mock_formatters();
    assert_eq!(apply_formatter("42", "pad( 4 , 0 )", &formatters), "0042");
    assert_eq!(apply_formatter("5", "multiply( 3 )", &formatters), "15");
}

// Invalid formatter strings tests
#[test]
fn test_return_string_value_for_invalid_formatter_syntax() {
    let formatters = create_mock_formatters();
    assert_eq!(apply_formatter("test", "invalid!@#", &formatters), "test");
    assert_eq!(apply_formatter("123", "bad-format", &formatters), "123");
    assert_eq!(apply_formatter("hello", "upper(", &formatters), "hello");
    assert_eq!(apply_formatter("world", "lower)", &formatters), "world");
}

#[test]
fn test_return_string_value_for_non_existent_formatter() {
    let formatters = create_mock_formatters();
    assert_eq!(apply_formatter("test", "nonexistent", &formatters), "test");
    assert_eq!(apply_formatter("456", "missing(arg)", &formatters), "456");
}

// Edge cases tests
#[test]
fn test_handle_empty_arguments() {
    let formatters = create_mock_formatters();
    assert_eq!(apply_formatter("test", "upper()", &formatters), "TEST");
}

#[test]
fn test_handle_empty_formatters_object() {
    let formatters: HashMap<String, Formatter> = HashMap::new();
    assert_eq!(apply_formatter("test", "upper", &formatters), "test");
    assert_eq!(apply_formatter("123", "nonexistent", &formatters), "123");
}

// Formatter name validation tests
#[test]
fn test_only_accept_word_characters_in_formatter_names() {
    let mut invalid_formatters: HashMap<String, Formatter> = HashMap::new();
    invalid_formatters.insert("with-dash".to_string(), Box::new(|v, _| v.to_string()));
    invalid_formatters.insert("with.dot".to_string(), Box::new(|v, _| v.to_string()));
    invalid_formatters.insert("with space".to_string(), Box::new(|v, _| v.to_string()));

    // These should not match due to invalid characters in formatter name
    assert_eq!(
        apply_formatter("test", "with-dash", &invalid_formatters),
        "test"
    );
    assert_eq!(
        apply_formatter("test", "with.dot", &invalid_formatters),
        "test"
    );
    assert_eq!(
        apply_formatter("test", "with space", &invalid_formatters),
        "test"
    );
}
