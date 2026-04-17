from src.number.format_number import format_number


def test_format_decimal_en_us():
    assert format_number(1234567.89) == "1,234,567.89"


def test_format_decimal_de_de():
    assert format_number(1234567.89, locale="de-DE") == "1.234.567,89"


def test_format_decimal_fallback_for_unknown_locale():
    # Unknown locales fall back to en-US layout.
    assert format_number(1234.5, locale="xx-YY") == "1,234.5"


def test_format_percent_default():
    assert format_number(0.75, style="percent") == "75%"


def test_format_percent_with_fraction_digits():
    assert (
        format_number(
            0.1234,
            style="percent",
            minimum_fraction_digits=2,
            maximum_fraction_digits=2,
        )
        == "12.34%"
    )


def test_format_currency_usd_en_us():
    assert (
        format_number(1234.5, style="currency", currency="USD", locale="en-US")
        == "$1,234.50"
    )


def test_format_currency_jpy_defaults_to_zero_fraction_digits():
    assert format_number(1234, style="currency", currency="JPY") == "\u00a51,234"


def test_format_currency_unknown_code_uses_iso_prefix():
    assert (
        format_number(1000, style="currency", currency="XYZ", locale="en-US")
        == "XYZ\u00a01,000.00"
    )


def test_minimum_fraction_digits_pads_trailing_zeros():
    assert format_number(1, minimum_fraction_digits=2) == "1.00"


def test_maximum_fraction_digits_truncates_with_half_away_from_zero():
    # 0.125 rounded to 2 digits should be 0.13 under half-away-from-zero.
    assert format_number(0.125, maximum_fraction_digits=2) == "0.13"
    # Negative mirror: -0.125 -> -0.13.
    assert format_number(-0.125, maximum_fraction_digits=2) == "-0.13"


def test_negative_numbers_keep_sign_and_separators():
    assert format_number(-1234567.8, locale="en-US") == "-1,234,567.8"


def test_fraction_bounds_normalize_when_max_less_than_min():
    # Mimic Intl.NumberFormat's forgiving normalization.
    assert (
        format_number(1.5, minimum_fraction_digits=3, maximum_fraction_digits=1)
        == "1.500"
    )
