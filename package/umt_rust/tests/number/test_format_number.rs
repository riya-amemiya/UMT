use umt_rust::number::{
    FormatNumberOptions, FormatStyle, umt_format_number, umt_format_number_default,
};

#[test]
fn should_format_decimal_en_us_by_default() {
    assert_eq!(umt_format_number_default(1234567.89), "1,234,567.89");
}

#[test]
fn should_format_decimal_with_de_de_locale() {
    let options = FormatNumberOptions {
        locale: Some("de-DE"),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(1234567.89, &options), "1.234.567,89");
}

#[test]
fn should_fall_back_to_en_us_layout_for_unknown_locale() {
    let options = FormatNumberOptions {
        locale: Some("xx-YY"),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(1234.5, &options), "1,234.5");
}

#[test]
fn should_format_percent_with_default_digits() {
    let options = FormatNumberOptions {
        style: Some(FormatStyle::Percent),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(0.75, &options), "75%");
}

#[test]
fn should_format_percent_with_explicit_fraction_digits() {
    let options = FormatNumberOptions {
        style: Some(FormatStyle::Percent),
        minimum_fraction_digits: Some(2),
        maximum_fraction_digits: Some(2),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(0.1234, &options), "12.34%");
}

#[test]
fn should_format_currency_usd_with_en_us() {
    let options = FormatNumberOptions {
        style: Some(FormatStyle::Currency),
        currency: Some("USD"),
        locale: Some("en-US"),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(1234.5, &options), "$1,234.50");
}

#[test]
fn should_default_jpy_currency_to_zero_fraction_digits() {
    let options = FormatNumberOptions {
        style: Some(FormatStyle::Currency),
        currency: Some("JPY"),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(1234.0, &options), "\u{00a5}1,234");
}

#[test]
fn should_prefix_unknown_currency_with_iso_code_and_nbsp() {
    let options = FormatNumberOptions {
        style: Some(FormatStyle::Currency),
        currency: Some("XYZ"),
        locale: Some("en-US"),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(1000.0, &options), "XYZ\u{00a0}1,000.00");
}

#[test]
fn should_pad_with_minimum_fraction_digits() {
    let options = FormatNumberOptions {
        minimum_fraction_digits: Some(2),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(1.0, &options), "1.00");
}

#[test]
fn should_truncate_with_half_away_from_zero_rounding() {
    let options = FormatNumberOptions {
        maximum_fraction_digits: Some(2),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(0.125, &options), "0.13");
    assert_eq!(umt_format_number(-0.125, &options), "-0.13");
}

#[test]
fn should_preserve_sign_and_separators_for_negative_numbers() {
    let options = FormatNumberOptions {
        locale: Some("en-US"),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(-1234567.8, &options), "-1,234,567.8");
}

#[test]
fn should_lift_max_fraction_digits_to_minimum() {
    let options = FormatNumberOptions {
        minimum_fraction_digits: Some(3),
        maximum_fraction_digits: Some(1),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(1.5, &options), "1.500");
}
