"""Locale-aware number formatting ported from package/main Number/formatNumber.ts."""

import math
from dataclasses import dataclass
from decimal import ROUND_HALF_UP, Context, Decimal
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

_ROUNDING_CONTEXT = Context(prec=500, rounding=ROUND_HALF_UP)


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


def _format_absolute(
    value: float,
    scale: int,
    min_fraction: int,
    max_fraction: int,
    spec: _LocaleSpec,
) -> str:
    if math.isnan(value):
        return "NaN"
    if math.isinf(value):
        return "\u221e"

    rounded = (
        Decimal(repr(value))
        .scaleb(scale, _ROUNDING_CONTEXT)
        .quantize(Decimal(1).scaleb(-max_fraction), context=_ROUNDING_CONTEXT)
    )
    integer_digits, _, fraction_digits = f"{rounded:f}".partition(".")
    integer_str = f"{int(integer_digits):,}".replace(",", spec.thousands)

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

    sign = "-" if not math.isnan(value) and math.copysign(1, value) < 0 else ""
    magnitude = _format_absolute(
        abs(float(value)),
        2 if style == "percent" else 0,
        min_fraction,
        max_fraction,
        spec,
    )

    if style == "percent":
        return f"{sign}{magnitude}%"

    if style == "currency":
        symbol = _CURRENCY_SYMBOLS.get(
            currency or "", f"{currency}\u00a0" if currency else ""
        )
        return f"{sign}{symbol}{magnitude}"

    return f"{sign}{magnitude}"
