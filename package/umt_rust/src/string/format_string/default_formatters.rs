use super::apply_formatter::Formatter;
use std::collections::HashMap;

/// Creates the default formatters for formatString.
///
/// Available formatters:
/// - upper: Convert to uppercase
/// - lower: Convert to lowercase
/// - pad: Pad string with characters to specified length
/// - plural: Choose singular/plural form based on count
pub fn create_default_formatters() -> HashMap<String, Formatter> {
    let mut formatters: HashMap<String, Formatter> = HashMap::new();

    // Upper case formatter
    formatters.insert(
        "upper".to_string(),
        Box::new(|value, _args| value.to_uppercase()),
    );

    // Lower case formatter
    formatters.insert(
        "lower".to_string(),
        Box::new(|value, _args| value.to_lowercase()),
    );

    // Plural formatter
    formatters.insert(
        "plural".to_string(),
        Box::new(|value, args| {
            let num: f64 = value.parse().unwrap_or(0.0);
            let singular = args.first().map(|s| s.as_str()).unwrap_or("");
            let plural = args.get(1).map(|s| s.as_str()).unwrap_or("");

            if num == 1.0 {
                singular.to_string()
            } else {
                plural.to_string()
            }
        }),
    );

    // Pad formatter
    formatters.insert(
        "pad".to_string(),
        Box::new(|value, args| {
            let length: usize = args
                .first()
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(2);
            let pad_char = args
                .get(1)
                .and_then(|s| {
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        trimmed.chars().next()
                    }
                })
                .unwrap_or('0');

            let current_len = value.len();
            if current_len >= length {
                return value.to_string();
            }

            let padding_needed = length - current_len;
            let padding: String = std::iter::repeat_n(pad_char, padding_needed).collect();
            format!("{}{}", padding, value)
        }),
    );

    // Number formatter (simplified version without locale support)
    formatters.insert(
        "number".to_string(),
        Box::new(|value, args| {
            let num: f64 = value.parse().unwrap_or(0.0);
            let min_fraction: usize = args.get(1).and_then(|s| s.trim().parse().ok()).unwrap_or(0);
            let max_fraction: usize = args
                .get(2)
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(20);

            // Format with maximum fraction digits, then trim trailing zeros if needed
            let formatted = format!("{:.prec$}", num, prec = max_fraction);
            let parts: Vec<&str> = formatted.split('.').collect();

            if parts.len() == 2 {
                let integer_part = parts[0];
                let decimal_part = parts[1].trim_end_matches('0');

                if decimal_part.is_empty() && min_fraction == 0 {
                    integer_part.to_string()
                } else if decimal_part.len() < min_fraction {
                    format!(
                        "{}.{:0<width$}",
                        integer_part,
                        decimal_part,
                        width = min_fraction
                    )
                } else {
                    format!("{}.{}", integer_part, decimal_part)
                }
            } else {
                formatted
            }
        }),
    );

    formatters
}
