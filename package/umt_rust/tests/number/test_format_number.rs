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

#[test]
fn should_return_nan_string_for_nan_input() {
    assert_eq!(umt_format_number_default(f64::NAN), "NaN");

    let percent = FormatNumberOptions {
        style: Some(FormatStyle::Percent),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(f64::NAN, &percent), "NaN%");

    let currency = FormatNumberOptions {
        style: Some(FormatStyle::Currency),
        currency: Some("USD"),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(f64::NAN, &currency), "$NaN");
}

#[test]
fn should_round_shortest_decimal_form_half_away_from_zero() {
    let two = FormatNumberOptions {
        maximum_fraction_digits: Some(2),
        ..FormatNumberOptions::default()
    };
    let one = FormatNumberOptions {
        maximum_fraction_digits: Some(1),
        ..FormatNumberOptions::default()
    };
    let zero = FormatNumberOptions {
        maximum_fraction_digits: Some(0),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(1.005, &two), "1.01");
    assert_eq!(umt_format_number(0.285, &two), "0.29");
    assert_eq!(umt_format_number(1.45, &one), "1.5");
    assert_eq!(umt_format_number(2.5, &zero), "3");
    assert_eq!(umt_format_number(-2.5, &zero), "-3");
    assert_eq!(umt_format_number(0.4, &zero), "0");
}

#[test]
fn should_carry_rounding_into_a_new_integer_digit() {
    let options = FormatNumberOptions {
        maximum_fraction_digits: Some(1),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(999.96, &options), "1,000");
    assert_eq!(umt_format_number(0.96, &options), "1");
    assert_eq!(umt_format_number(19.96, &options), "20");
}

#[test]
fn should_scale_percent_without_binary_error() {
    let options = FormatNumberOptions {
        style: Some(FormatStyle::Percent),
        maximum_fraction_digits: Some(20),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(0.07, &options), "7%");
    assert_eq!(umt_format_number(0.005, &options), "0.5%");
    assert_eq!(umt_format_number(12.0, &options), "1,200%");
}

#[test]
fn should_format_nan_with_style_affixes() {
    let percent = FormatNumberOptions {
        style: Some(FormatStyle::Percent),
        ..FormatNumberOptions::default()
    };
    let currency = FormatNumberOptions {
        style: Some(FormatStyle::Currency),
        currency: Some("USD"),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number_default(f64::NAN), "NaN");
    assert_eq!(umt_format_number_default(-f64::NAN), "NaN");
    assert_eq!(umt_format_number(f64::NAN, &percent), "NaN%");
    assert_eq!(umt_format_number(f64::NAN, &currency), "$NaN");
}

#[test]
fn should_format_infinity_as_infinity_sign() {
    let percent = FormatNumberOptions {
        style: Some(FormatStyle::Percent),
        ..FormatNumberOptions::default()
    };
    let currency = FormatNumberOptions {
        style: Some(FormatStyle::Currency),
        currency: Some("USD"),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number_default(f64::INFINITY), "\u{221e}");
    assert_eq!(umt_format_number_default(f64::NEG_INFINITY), "-\u{221e}");
    assert_eq!(umt_format_number(f64::INFINITY, &percent), "\u{221e}%");
    assert_eq!(umt_format_number(f64::INFINITY, &currency), "$\u{221e}");
}

#[test]
fn should_keep_sign_of_negative_zero() {
    assert_eq!(umt_format_number_default(-0.0), "-0");
    assert_eq!(umt_format_number_default(-0.0001), "-0");
    assert_eq!(umt_format_number_default(0.0), "0");
}

#[test]
fn should_keep_every_digit_of_large_and_small_values() {
    let small = FormatNumberOptions {
        maximum_fraction_digits: Some(10),
        ..FormatNumberOptions::default()
    };
    assert_eq!(
        umt_format_number_default(1e21),
        "1,000,000,000,000,000,000,000"
    );
    assert_eq!(
        umt_format_number_default(123456789012345680000.0),
        "123,456,789,012,345,680,000"
    );
    assert_eq!(umt_format_number(1e-7, &small), "0.0000001");
}

#[test]
fn should_format_currency_without_code_and_with_zero_fraction_code() {
    let no_code = FormatNumberOptions {
        style: Some(FormatStyle::Currency),
        ..FormatNumberOptions::default()
    };
    let jpy = FormatNumberOptions {
        style: Some(FormatStyle::Currency),
        currency: Some("JPY"),
        ..FormatNumberOptions::default()
    };
    let usd = FormatNumberOptions {
        style: Some(FormatStyle::Currency),
        currency: Some("USD"),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(1.5, &no_code), "1.50");
    assert_eq!(umt_format_number(1.5, &jpy), "\u{00a5}2");
    assert_eq!(umt_format_number(-5.0, &usd), "-$5.00");
}

#[test]
fn should_resolve_locale_by_language_prefix() {
    let de = FormatNumberOptions {
        locale: Some("de-AT"),
        ..FormatNumberOptions::default()
    };
    let fr = FormatNumberOptions {
        locale: Some("fr-FR"),
        ..FormatNumberOptions::default()
    };
    assert_eq!(umt_format_number(1234.5, &de), "1.234,5");
    assert_eq!(umt_format_number(1234.5, &fr), "1\u{202f}234,5");
}
