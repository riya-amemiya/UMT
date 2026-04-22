use std::collections::HashMap;

/// Formatting style accepted by [`umt_format_number`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormatStyle {
    #[default]
    Decimal,
    Currency,
    Percent,
}

/// Options mirroring the `Intl.NumberFormat`-like API exposed by
/// `package/main` and `umt_python`.
#[derive(Debug, Default, Clone)]
pub struct FormatNumberOptions {
    /// BCP 47 locale tag (e.g. `"en-US"`, `"de-DE"`).
    pub locale: Option<String>,
    /// Minimum number of fractional digits.
    pub minimum_fraction_digits: Option<u32>,
    /// Maximum number of fractional digits.
    pub maximum_fraction_digits: Option<u32>,
    /// Formatting style. Defaults to [`FormatStyle::Decimal`].
    pub style: FormatStyle,
    /// ISO 4217 currency code used when `style` is [`FormatStyle::Currency`].
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Copy)]
struct LocaleSpec {
    thousands: &'static str,
    decimal: &'static str,
}

const DEFAULT_LOCALE_SPEC: LocaleSpec = LocaleSpec {
    thousands: ",",
    decimal: ".",
};

fn locale_separators() -> &'static [(&'static str, LocaleSpec)] {
    // Keep in sync with umt_python/src/number/format_number.py
    &[
        (
            "en-US",
            LocaleSpec {
                thousands: ",",
                decimal: ".",
            },
        ),
        (
            "en-GB",
            LocaleSpec {
                thousands: ",",
                decimal: ".",
            },
        ),
        (
            "ja-JP",
            LocaleSpec {
                thousands: ",",
                decimal: ".",
            },
        ),
        (
            "de-DE",
            LocaleSpec {
                thousands: ".",
                decimal: ",",
            },
        ),
        (
            "fr-FR",
            LocaleSpec {
                thousands: "\u{202f}",
                decimal: ",",
            },
        ),
        (
            "it-IT",
            LocaleSpec {
                thousands: ".",
                decimal: ",",
            },
        ),
        (
            "es-ES",
            LocaleSpec {
                thousands: ".",
                decimal: ",",
            },
        ),
    ]
}

fn currency_symbols() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("USD", "$"),
        ("EUR", "\u{20ac}"),
        ("GBP", "\u{00a3}"),
        ("JPY", "\u{00a5}"),
        ("CNY", "\u{00a5}"),
        ("KRW", "\u{20a9}"),
    ])
}

// Currencies whose Intl.NumberFormat default fraction digits diverge from
// the standard "2 digits" rule and use 0 instead.
const ZERO_FRACTION_CURRENCIES: &[&str] = &["JPY", "KRW"];

fn resolve_locale_spec(locale: Option<&str>) -> LocaleSpec {
    let Some(tag) = locale else {
        return DEFAULT_LOCALE_SPEC;
    };
    let separators = locale_separators();
    for (key, spec) in separators {
        if *key == tag {
            return *spec;
        }
    }
    let language = tag.split('-').next().unwrap_or(tag);
    for (key, spec) in separators {
        if key.split('-').next().unwrap_or(*key) == language {
            return *spec;
        }
    }
    DEFAULT_LOCALE_SPEC
}

fn default_fraction_digits(style: FormatStyle, currency: Option<&str>) -> (u32, u32) {
    match style {
        FormatStyle::Currency => {
            if currency
                .map(|c| ZERO_FRACTION_CURRENCIES.contains(&c))
                .unwrap_or(false)
            {
                (0, 0)
            } else {
                (2, 2)
            }
        }
        FormatStyle::Percent => (0, 0),
        FormatStyle::Decimal => (0, 3),
    }
}

// Python's round() uses banker's rounding; Intl.NumberFormat rounds half
// away from zero, so we cannot delegate to Rust's f64::round either without
// care. Rust's `round` already rounds half away from zero, but we scale
// manually to avoid surprises from floating point drift at boundary cases.
fn round_half_away_from_zero(value: f64, digits: u32) -> f64 {
    let factor = 10_f64.powi(digits as i32);
    let scaled = value * factor;
    let rounded = if scaled >= 0.0 {
        (scaled + 0.5).floor()
    } else {
        -((-scaled + 0.5).floor())
    };
    rounded / factor
}

fn insert_thousands_separators(integer_part: u64, separator: &str) -> String {
    let raw = integer_part.to_string();
    if separator.is_empty() || raw.len() <= 3 {
        return raw;
    }
    let bytes = raw.as_bytes();
    let mut out = String::with_capacity(raw.len() + raw.len() / 3);
    let remainder = bytes.len() % 3;
    if remainder > 0 {
        out.push_str(&raw[..remainder]);
    }
    let mut idx = remainder;
    while idx < bytes.len() {
        if idx != 0 {
            out.push_str(separator);
        }
        out.push_str(&raw[idx..idx + 3]);
        idx += 3;
    }
    out
}

fn format_absolute(value: f64, min_fraction: u32, max_fraction: u32, spec: LocaleSpec) -> String {
    let rounded = round_half_away_from_zero(value, max_fraction);
    let integer_part = rounded.trunc() as u64;
    let remainder = (rounded - integer_part as f64).abs();

    let integer_str = insert_thousands_separators(integer_part, spec.thousands);

    if max_fraction == 0 {
        return integer_str;
    }

    // Format the fractional part with enough precision, then trim trailing
    // zeros down to `min_fraction`, matching Intl.NumberFormat behaviour.
    let fraction_all = format!("{:.*}", max_fraction as usize, remainder);
    // Skip the leading "0." (or "1." when carry occurred; carry is already
    // absorbed into integer_part by the rounding above so "0." is expected).
    let fraction_digits = fraction_all
        .split_once('.')
        .map(|(_, rest)| rest.to_string())
        .unwrap_or_default();
    let trimmed = fraction_digits.trim_end_matches('0');
    let trimmed: String = if (trimmed.len() as u32) < min_fraction {
        fraction_digits
            .chars()
            .take(min_fraction as usize)
            .collect()
    } else {
        trimmed.to_string()
    };
    if trimmed.is_empty() {
        return integer_str;
    }
    format!("{}{}{}", integer_str, spec.decimal, trimmed)
}

/// Formats a number similarly to `Intl.NumberFormat` in the TypeScript source.
///
/// # Arguments
///
/// * `value` - Numeric value to format.
/// * `options` - Formatting options. See [`FormatNumberOptions`].
///
/// # Example
///
/// ```
/// use umt_rust::number::{umt_format_number, FormatNumberOptions, FormatStyle};
///
/// assert_eq!(
///     umt_format_number(1_234_567.89, &FormatNumberOptions::default()),
///     "1,234,567.89"
/// );
///
/// let opts = FormatNumberOptions {
///     locale: Some("de-DE".to_string()),
///     ..Default::default()
/// };
/// assert_eq!(umt_format_number(1_234_567.89, &opts), "1.234.567,89");
///
/// let opts = FormatNumberOptions {
///     style: FormatStyle::Percent,
///     ..Default::default()
/// };
/// assert_eq!(umt_format_number(0.75, &opts), "75%");
///
/// let opts = FormatNumberOptions {
///     style: FormatStyle::Currency,
///     currency: Some("USD".to_string()),
///     locale: Some("en-US".to_string()),
///     ..Default::default()
/// };
/// assert_eq!(umt_format_number(1234.5, &opts), "$1,234.50");
/// ```
pub fn umt_format_number(value: f64, options: &FormatNumberOptions) -> String {
    let style = options.style;
    let currency = options.currency.as_deref();

    let (default_min, default_max) = default_fraction_digits(style, currency);
    let min_fraction = options.minimum_fraction_digits.unwrap_or(default_min);
    let max_fraction = options.maximum_fraction_digits.unwrap_or(default_max);
    // Intl.NumberFormat forgives max < min by clamping max up to min.
    let max_fraction = max_fraction.max(min_fraction);

    let spec = resolve_locale_spec(options.locale.as_deref());

    let scaled_value = if matches!(style, FormatStyle::Percent) {
        value * 100.0
    } else {
        value
    };

    let sign = if scaled_value < 0.0 { "-" } else { "" };
    let magnitude = format_absolute(scaled_value.abs(), min_fraction, max_fraction, spec);

    match style {
        FormatStyle::Percent => format!("{}{}%", sign, magnitude),
        FormatStyle::Currency => {
            let symbol_map = currency_symbols();
            let code = currency.unwrap_or("");
            let symbol_owned: String = if let Some(sym) = symbol_map.get(code) {
                (*sym).to_string()
            } else if !code.is_empty() {
                format!("{}\u{00a0}", code)
            } else {
                String::new()
            };
            format!("{}{}{}", sign, symbol_owned, magnitude)
        }
        FormatStyle::Decimal => format!("{}{}", sign, magnitude),
    }
}

/// Convenience wrapper that formats with the default options.
pub fn umt_format_number_default(value: f64) -> String {
    umt_format_number(value, &FormatNumberOptions::default())
}
