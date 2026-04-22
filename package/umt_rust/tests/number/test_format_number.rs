use umt_rust::number::{
    FormatNumberOptions, FormatStyle, umt_format_number, umt_format_number_default,
};

fn opts_with_locale(locale: &str) -> FormatNumberOptions {
    FormatNumberOptions {
        locale: Some(locale.to_string()),
        ..Default::default()
    }
}

#[test]
fn test_format_decimal_en_us() {
    assert_eq!(
        umt_format_number(1_234_567.89, &opts_with_locale("en-US")),
        "1,234,567.89"
    );
}

#[test]
fn test_format_decimal_de_de() {
    assert_eq!(
        umt_format_number(1_234_567.89, &opts_with_locale("de-DE")),
        "1.234.567,89"
    );
}

#[test]
fn test_format_decimal_fallback_for_unknown_locale() {
    // Unknown locales fall back to en-US layout (mirrors umt_python).
    assert_eq!(
        umt_format_number(1234.5, &opts_with_locale("xx-YY")),
        "1,234.5"
    );
}

#[test]
fn test_format_percent_default() {
    let opts = FormatNumberOptions {
        style: FormatStyle::Percent,
        ..Default::default()
    };
    assert_eq!(umt_format_number(0.75, &opts), "75%");
}

#[test]
fn test_format_percent_with_fraction_digits() {
    let opts = FormatNumberOptions {
        style: FormatStyle::Percent,
        minimum_fraction_digits: Some(2),
        maximum_fraction_digits: Some(2),
        ..Default::default()
    };
    assert_eq!(umt_format_number(0.1234, &opts), "12.34%");
}

#[test]
fn test_format_currency_usd_en_us() {
    let opts = FormatNumberOptions {
        style: FormatStyle::Currency,
        currency: Some("USD".to_string()),
        locale: Some("en-US".to_string()),
        ..Default::default()
    };
    assert_eq!(umt_format_number(1234.5, &opts), "$1,234.50");
}

#[test]
fn test_format_currency_jpy_defaults_to_zero_fraction_digits() {
    let opts = FormatNumberOptions {
        style: FormatStyle::Currency,
        currency: Some("JPY".to_string()),
        ..Default::default()
    };
    assert_eq!(umt_format_number(1234.0, &opts), "\u{00a5}1,234");
}

#[test]
fn test_format_currency_unknown_code_uses_iso_prefix() {
    let opts = FormatNumberOptions {
        style: FormatStyle::Currency,
        currency: Some("XYZ".to_string()),
        locale: Some("en-US".to_string()),
        ..Default::default()
    };
    assert_eq!(umt_format_number(1000.0, &opts), "XYZ\u{00a0}1,000.00");
}

#[test]
fn test_minimum_fraction_digits_pads_trailing_zeros() {
    let opts = FormatNumberOptions {
        minimum_fraction_digits: Some(2),
        ..Default::default()
    };
    assert_eq!(umt_format_number(1.0, &opts), "1.00");
}

#[test]
fn test_maximum_fraction_digits_truncates_with_half_away_from_zero() {
    let opts = FormatNumberOptions {
        maximum_fraction_digits: Some(2),
        ..Default::default()
    };
    // 0.125 rounded to 2 digits should be 0.13 under half-away-from-zero.
    assert_eq!(umt_format_number(0.125, &opts), "0.13");
    // Negative mirror: -0.125 -> -0.13.
    assert_eq!(umt_format_number(-0.125, &opts), "-0.13");
}

#[test]
fn test_negative_numbers_keep_sign_and_separators() {
    assert_eq!(
        umt_format_number(-1_234_567.8, &opts_with_locale("en-US")),
        "-1,234,567.8"
    );
}

#[test]
fn test_fraction_bounds_normalize_when_max_less_than_min() {
    // Mimic Intl.NumberFormat's forgiving normalization: max gets clamped
    // up to min when max < min is supplied.
    let opts = FormatNumberOptions {
        minimum_fraction_digits: Some(3),
        maximum_fraction_digits: Some(1),
        ..Default::default()
    };
    assert_eq!(umt_format_number(1.5, &opts), "1.500");
}

#[test]
fn test_zero_formats_without_fractional_part() {
    assert_eq!(umt_format_number(0.0, &opts_with_locale("en-US")), "0");
}

#[test]
fn test_default_wrapper_matches_explicit_default() {
    assert_eq!(
        umt_format_number_default(1234.5),
        umt_format_number(1234.5, &FormatNumberOptions::default())
    );
}
