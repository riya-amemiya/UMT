// Locale-aware number formatting ported from package/main Number/formatNumber.ts.
//
// The TypeScript reference delegates to `Intl.NumberFormat`, which Rust lacks.
// We reproduce the same observable output for the locales and currencies that
// the Python port (`package/umt_python/src/number/format_number.py`) already
// covers, keeping the three packages in sync.

/// Formatting style accepted by [`umt_format_number`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormatStyle {
    #[default]
    Decimal,
    Currency,
    Percent,
}

/// Options mirroring the TypeScript `FormatNumberOptions` interface.
///
/// Using `Option` for each field allows callers to rely on
/// `Intl.NumberFormat`-style defaults while still overriding individual knobs.
#[derive(Debug, Clone, Default)]
pub struct FormatNumberOptions<'a> {
    /// BCP 47 locale tag (e.g. "en-US", "de-DE"). Defaults to "en-US" semantics.
    pub locale: Option<&'a str>,
    /// Minimum number of fractional digits.
    pub minimum_fraction_digits: Option<u32>,
    /// Maximum number of fractional digits.
    pub maximum_fraction_digits: Option<u32>,
    /// Formatting style (decimal / currency / percent).
    pub style: Option<FormatStyle>,
    /// ISO 4217 code used when `style` is `Currency`.
    pub currency: Option<&'a str>,
}

struct LocaleSpec {
    thousands: &'static str,
    decimal: &'static str,
}

const EN_SPEC: LocaleSpec = LocaleSpec {
    thousands: ",",
    decimal: ".",
};
const DE_SPEC: LocaleSpec = LocaleSpec {
    thousands: ".",
    decimal: ",",
};
const FR_SPEC: LocaleSpec = LocaleSpec {
    thousands: "\u{202f}",
    decimal: ",",
};

fn resolve_locale_spec(locale: Option<&str>) -> LocaleSpec {
    match locale {
        None => EN_SPEC,
        Some(tag) => {
            // Explicit match first, then language-only fallback, then en-US.
            match tag {
                "en-US" | "en-GB" | "ja-JP" => EN_SPEC,
                "de-DE" | "it-IT" | "es-ES" => DE_SPEC,
                "fr-FR" => FR_SPEC,
                _ => match tag.split('-').next().unwrap_or("") {
                    "en" | "ja" => EN_SPEC,
                    "de" | "it" | "es" => DE_SPEC,
                    "fr" => FR_SPEC,
                    _ => EN_SPEC,
                },
            }
        }
    }
}

fn currency_symbol(code: &str) -> Option<&'static str> {
    // Matches the symbol table used by the Python port.
    match code {
        "USD" => Some("$"),
        "EUR" => Some("\u{20ac}"),
        "GBP" => Some("\u{00a3}"),
        "JPY" | "CNY" => Some("\u{00a5}"),
        "KRW" => Some("\u{20a9}"),
        _ => None,
    }
}

// Currencies whose Intl defaults are 0 fraction digits rather than 2.
fn is_zero_fraction_currency(code: &str) -> bool {
    matches!(code, "JPY" | "KRW")
}

fn default_fraction_digits(style: FormatStyle, currency: Option<&str>) -> (u32, u32) {
    match style {
        FormatStyle::Currency => {
            if currency.is_some_and(is_zero_fraction_currency) {
                (0, 0)
            } else {
                (2, 2)
            }
        }
        FormatStyle::Percent => (0, 0),
        FormatStyle::Decimal => (0, 3),
    }
}

fn insert_thousands(digits: &str, sep: &str) -> String {
    let bytes = digits.as_bytes();
    let n = bytes.len();
    let mut out = String::with_capacity(n + n / 3 * sep.len());
    for (i, byte) in bytes.iter().enumerate() {
        if i > 0 && (n - i).is_multiple_of(3) {
            out.push_str(sep);
        }
        out.push(*byte as char);
    }
    out
}

// Intl.NumberFormat rounds the shortest decimal form of the number half away
// from zero, so the digits are rounded as decimals instead of as binary
// floats. Returns the integer digits and exactly `max_fraction` fraction
// digits of `value * 10^scale`.
fn round_decimal_digits(value: f64, scale: usize, max_fraction: usize) -> (String, String) {
    let text = value.to_string();
    let (integer_text, fraction_text) = text.split_once('.').unwrap_or((&text, ""));
    let mut digits: Vec<u8> = integer_text
        .bytes()
        .chain(fraction_text.bytes())
        .map(|byte| byte - b'0')
        .collect();
    let mut point = integer_text.len() + scale;
    if digits.len() < point + max_fraction + 1 {
        digits.resize(point + max_fraction + 1, 0);
    }

    let round_up = digits[point + max_fraction] >= 5;
    digits.truncate(point + max_fraction);
    if round_up {
        let mut index = digits.len();
        loop {
            if index == 0 {
                digits.insert(0, 1);
                point += 1;
                break;
            }
            index -= 1;
            if digits[index] == 9 {
                digits[index] = 0;
            } else {
                digits[index] += 1;
                break;
            }
        }
    }

    let leading_zeros = digits[..point]
        .iter()
        .take(point.saturating_sub(1))
        .take_while(|digit| **digit == 0)
        .count();
    let to_text = |part: &[u8]| part.iter().map(|digit| (digit + b'0') as char).collect();
    (
        to_text(&digits[leading_zeros..point]),
        to_text(&digits[point..]),
    )
}

fn format_absolute(
    value: f64,
    scale: usize,
    min_fraction: u32,
    max_fraction: u32,
    spec: &LocaleSpec,
) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return "\u{221e}".to_string();
    }

    let (integer_digits, fraction_digits) =
        round_decimal_digits(value, scale, max_fraction as usize);
    let integer_str = insert_thousands(&integer_digits, spec.thousands);

    let trimmed = fraction_digits.trim_end_matches('0');
    let min = min_fraction as usize;
    let final_fraction = if trimmed.len() < min {
        &fraction_digits[..min]
    } else {
        trimmed
    };

    if final_fraction.is_empty() {
        integer_str
    } else {
        format!("{}{}{}", integer_str, spec.decimal, final_fraction)
    }
}

/// Formats `value` according to `options`, mirroring `Intl.NumberFormat`.
///
/// NaN formats as `"NaN"` and infinities as `"\u{221e}"`, with the sign,
/// percent sign and currency symbol applied as for finite values.
///
/// # Arguments
///
/// * `value` - The number to format.
/// * `options` - Locale, style, currency, and fraction-digit configuration.
///
/// # Returns
///
/// The formatted number as a `String`.
///
/// # Examples
///
/// ```
/// use umt_rust::number::{umt_format_number, FormatNumberOptions, FormatStyle};
///
/// assert_eq!(
///     umt_format_number(1234567.89, &FormatNumberOptions::default()),
///     "1,234,567.89",
/// );
///
/// let de = FormatNumberOptions {
///     locale: Some("de-DE"),
///     ..FormatNumberOptions::default()
/// };
/// assert_eq!(umt_format_number(1234567.89, &de), "1.234.567,89");
///
/// let percent = FormatNumberOptions {
///     style: Some(FormatStyle::Percent),
///     ..FormatNumberOptions::default()
/// };
/// assert_eq!(umt_format_number(0.75, &percent), "75%");
///
/// let usd = FormatNumberOptions {
///     style: Some(FormatStyle::Currency),
///     currency: Some("USD"),
///     locale: Some("en-US"),
///     ..FormatNumberOptions::default()
/// };
/// assert_eq!(umt_format_number(1234.5, &usd), "$1,234.50");
/// ```
pub fn umt_format_number(value: f64, options: &FormatNumberOptions) -> String {
    let style = options.style.unwrap_or_default();
    let (default_min, default_max) = default_fraction_digits(style, options.currency);
    let min_fraction = options.minimum_fraction_digits.unwrap_or(default_min);
    // Intl.NumberFormat forgives max < min by lifting max up to min.
    let max_fraction = options
        .maximum_fraction_digits
        .unwrap_or(default_max)
        .max(min_fraction);

    let spec = resolve_locale_spec(options.locale);

    let sign = if value.is_sign_negative() && !value.is_nan() {
        "-"
    } else {
        ""
    };
    let scale = if style == FormatStyle::Percent { 2 } else { 0 };
    let magnitude = format_absolute(value.abs(), scale, min_fraction, max_fraction, &spec);

    match style {
        FormatStyle::Percent => format!("{}{}%", sign, magnitude),
        FormatStyle::Currency => {
            let symbol = match options.currency {
                Some(code) => match currency_symbol(code) {
                    Some(sym) => sym.to_string(),
                    // ISO fallback uses a non-breaking space, as Intl does.
                    None => format!("{}\u{00a0}", code),
                },
                None => String::new(),
            };
            format!("{}{}{}", sign, symbol, magnitude)
        }
        FormatStyle::Decimal => format!("{}{}", sign, magnitude),
    }
}

/// Convenience wrapper that formats with all defaults (decimal, en-US layout).
///
/// # Arguments
///
/// * `value` - The number to format.
///
/// # Returns
///
/// The formatted number as a `String`.
pub fn umt_format_number_default(value: f64) -> String {
    umt_format_number(value, &FormatNumberOptions::default())
}
