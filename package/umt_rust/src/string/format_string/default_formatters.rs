use super::apply_formatter::Formatter;
use super::js_number;
use crate::number::{FormatNumberOptions, FormatStyle, umt_format_number};
use crate::string::umt_pad_start;
use std::collections::HashMap;

/// Creates the default formatters for formatString.
///
/// Available formatters:
/// - upper: Convert to uppercase
/// - lower: Convert to lowercase
/// - currency: Format as currency with locale support
/// - number: Format numbers with locale grouping and precision control
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

    // Currency formatter
    formatters.insert(
        "currency".to_string(),
        Box::new(|value, args| {
            umt_format_number(
                js_number(value),
                &FormatNumberOptions {
                    locale: Some(args.first().map_or("en-US", |s| s.as_str())),
                    style: Some(FormatStyle::Currency),
                    currency: Some(args.get(1).map_or("USD", |s| s.as_str())),
                    ..FormatNumberOptions::default()
                },
            )
        }),
    );

    // Plural formatter
    formatters.insert(
        "plural".to_string(),
        Box::new(|value, args| {
            let singular = args.first().map(|s| s.as_str()).unwrap_or("");
            let plural = args.get(1).map(|s| s.as_str()).unwrap_or("");

            if js_number(value) == 1.0 {
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
            let length = args.first().map_or(2.0, |s| js_number(s));
            let target_length = if length.is_finite() && length > 0.0 {
                length as usize
            } else {
                0
            };
            umt_pad_start(
                value,
                target_length,
                args.get(1).map_or("0", |s| s.as_str()),
            )
        }),
    );

    // Number formatter
    formatters.insert(
        "number".to_string(),
        Box::new(|value, args| {
            umt_format_number(
                js_number(value),
                &FormatNumberOptions {
                    locale: Some(args.first().map_or("en-US", |s| s.as_str())),
                    minimum_fraction_digits: Some(
                        args.get(1).and_then(|s| s.trim().parse().ok()).unwrap_or(0),
                    ),
                    maximum_fraction_digits: Some(
                        args.get(2)
                            .and_then(|s| s.trim().parse().ok())
                            .unwrap_or(20),
                    ),
                    ..FormatNumberOptions::default()
                },
            )
        }),
    );

    formatters
}
