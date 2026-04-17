"""Locale-aware number formatting ported from package/main Number/formatNumber.ts."""

from dataclasses import dataclass
from typing import Literal

FormatStyle = Literal["decimal", "currency", "percent"]


@dataclass(frozen=True)
class _LocaleSpec:
    thousands: str
    decimal: str


_LOCALE_SEPARATORS: dict[str, _LocaleSpec] = {
    "en-US": _LocaleSpec(thousands=",", decimal="."),
    "en-GB": _LocaleSpec(thousands=",", decimal="."),
    "ja-JP": _LocaleSpec(thousands=",", decimal="."),
    "de-DE": _LocaleSpec(thousands=".", decimal=","),
    "fr-FR": _LocaleSpec(thousands="\u202f", decimal=","),
    "it-IT": _LocaleSpec(thousands=".", decimal=","),
    "es-ES": _LocaleSpec(thousands=".", decimal=","),
}

_DEFAULT_LOCALE_SPEC = _LOCALE_SEPARATORS["en-US"]

_CURRENCY_SYMBOLS: dict[str, str] = {
    "USD": "$",
    "EUR": "\u20ac",
    "GBP": "\u00a3",
    "JPY": "\u00a5",
    "CNY": "\u00a5",
    "KRW": "\u20a9",
}

# Currencies whose Intl.NumberFormat default minimum/maximum fraction digits
# diverge from the standard "2 digits" rule and use 0 instead.
_ZERO_FRACTION_CURRENCIES = frozenset({"JPY", "KRW"})


def _resolve_locale_spec(locale: str | None) -> _LocaleSpec:
    if locale is None:
        return _DEFAULT_LOCALE_SPEC
    if locale in _LOCALE_SEPARATORS:
        return _LOCALE_SEPARATORS[locale]
    language = locale.split("-", 1)[0]
    for key, spec in _LOCALE_SEPARATORS.items():
        if key.split("-", 1)[0] == language:
            return spec
    return _DEFAULT_LOCALE_SPEC


def _default_fraction_digits(
    style: FormatStyle,
    currency: str | None,
) -> tuple[int, int]:
    if style == "currency":
        if currency in _ZERO_FRACTION_CURRENCIES:
            return (0, 0)
        return (2, 2)
    if style == "percent":
        return (0, 0)
    return (0, 3)


def _round_half_away_from_zero(value: float, digits: int) -> float:
    if digits < 0:
        msg = "digits must be non-negative"
        raise ValueError(msg)
    factor = 10**digits
    scaled = value * factor
    # Python's round() uses banker's rounding; Intl.NumberFormat rounds
    # half away from zero, so we cannot delegate to round() here.
    rounded = int(scaled + 0.5) if scaled >= 0 else -int(-scaled + 0.5)
    return rounded / factor


def _format_absolute(
    value: float,
    min_fraction: int,
    max_fraction: int,
    spec: _LocaleSpec,
) -> str:
    rounded = _round_half_away_from_zero(value, max_fraction)
    integer_part = int(rounded)
    remainder = abs(rounded - integer_part)

    integer_str = f"{integer_part:,}".replace(",", spec.thousands)

    if max_fraction == 0:
        return integer_str

    fraction_digits = f"{remainder:.{max_fraction}f}"[2:]
    trimmed = fraction_digits.rstrip("0")
    if len(trimmed) < min_fraction:
        trimmed = fraction_digits[:min_fraction]
    if not trimmed:
        return integer_str
    return f"{integer_str}{spec.decimal}{trimmed}"


def format_number(
    value: float | int,
    *,
    locale: str | None = None,
    minimum_fraction_digits: int | None = None,
    maximum_fraction_digits: int | None = None,
    style: FormatStyle = "decimal",
    currency: str | None = None,
) -> str:
    """Format a number similarly to ``Intl.NumberFormat`` in the TypeScript source.

    Args:
        value: Numeric value to format.
        locale: BCP 47 locale tag (e.g. ``"en-US"``, ``"de-DE"``). Defaults
            to ``"en-US"`` semantics when ``None``.
        minimum_fraction_digits: Minimum number of fractional digits.
        maximum_fraction_digits: Maximum number of fractional digits.
        style: One of ``"decimal"``, ``"currency"``, or ``"percent"``.
        currency: ISO 4217 code used when ``style`` is ``"currency"``.

    Returns:
        The formatted number as a string.

    Example:
        >>> format_number(1234567.89)
        '1,234,567.89'
        >>> format_number(1234567.89, locale="de-DE")
        '1.234.567,89'
        >>> format_number(0.75, style="percent")
        '75%'
        >>> format_number(1234.5, style="currency", currency="USD", locale="en-US")
        '$1,234.50'
    """

    default_min, default_max = _default_fraction_digits(style, currency)
    min_fraction = (
        default_min if minimum_fraction_digits is None else minimum_fraction_digits
    )
    max_fraction = (
        default_max if maximum_fraction_digits is None else maximum_fraction_digits
    )
    max_fraction = max(max_fraction, min_fraction)

    spec = _resolve_locale_spec(locale)

    scaled_value = value * 100 if style == "percent" else value
    sign = "-" if scaled_value < 0 else ""
    magnitude = _format_absolute(abs(scaled_value), min_fraction, max_fraction, spec)

    if style == "percent":
        return f"{sign}{magnitude}%"

    if style == "currency":
        symbol = _CURRENCY_SYMBOLS.get(
            currency or "", f"{currency}\u00a0" if currency else ""
        )
        return f"{sign}{symbol}{magnitude}"

    return f"{sign}{magnitude}"
