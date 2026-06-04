import math
from datetime import datetime, timezone
from typing import Any

from ..number.format_number import format_number
from .apply_formatter import Formatter, js_string
from .pad_start import pad_start


def _js_number(value: Any) -> float:  # noqa: ANN401
    """Coerces a value to a float following JavaScript ``Number()`` rules.

    Args:
        value: Value to coerce.

    Returns:
        The numeric coercion, or ``nan`` when coercion fails (JavaScript
        ``NaN``).
    """
    if value is None:
        return 0.0
    if isinstance(value, bool):
        return 1.0 if value else 0.0
    if isinstance(value, (int, float)):
        return float(value)
    if isinstance(value, str):
        stripped = value.strip()
        if stripped == "":
            return 0.0
        try:
            return float(stripped)
        except ValueError:
            return math.nan
    if isinstance(value, list):
        if len(value) == 0:
            return 0.0
        if len(value) == 1:
            return _js_number(value[0])
        return math.nan
    return math.nan


def _coerce_date(value: Any) -> datetime:  # noqa: ANN401
    """Resolves a value into a ``datetime``, mirroring ``new Date(...)``.

    Args:
        value: A ``datetime`` instance or a value parsable as an ISO date.

    Returns:
        The resolved ``datetime``.
    """
    if isinstance(value, datetime):
        return value
    return datetime.fromisoformat(js_string(value))


def _upper(value: Any) -> str:  # noqa: ANN401
    return js_string(value).upper()


def _lower(value: Any) -> str:  # noqa: ANN401
    return js_string(value).lower()


def _currency(
    value: Any,  # noqa: ANN401
    locale: str = "en-US",
    currency: str = "USD",
) -> str:
    return format_number(
        _js_number(value),
        locale=locale,
        style="currency",
        currency=currency,
    )


def _date(
    value: Any,  # noqa: ANN401
    locale: str = "en-US",  # noqa: ARG001
    format_: str = "short",
) -> str:
    date = _coerce_date(value)
    if format_ == "iso":
        utc = date.astimezone(timezone.utc) if date.tzinfo is not None else date
        return f"{utc.strftime('%Y-%m-%dT%H:%M:%S')}.{utc.microsecond // 1000:03d}Z"
    if format_ == "time":
        return date.strftime("%H:%M:%S")
    return date.strftime("%m/%d/%Y")


def _time(value: Any, locale: str = "en-US") -> str:  # noqa: ANN401, ARG001
    return _coerce_date(value).strftime("%H:%M:%S")


def _number(
    value: Any,  # noqa: ANN401
    locale: str = "en-US",
    minimum_fraction_digits: str = "0",
    maximum_fraction_digits: str = "20",
) -> str:
    return format_number(
        _js_number(value),
        locale=locale,
        minimum_fraction_digits=int(minimum_fraction_digits),
        maximum_fraction_digits=int(maximum_fraction_digits),
    )


def _plural(value: Any, singular: str, plural: str) -> str:  # noqa: ANN401
    return singular if _js_number(value) == 1 else plural


def _pad(value: Any, length: str = "2", char: str = "0") -> str:  # noqa: ANN401
    return pad_start(js_string(value), int(length), char)


# Built-in formatter functions for formatString.
#
# Each formatter takes a value and optional arguments, returning a formatted
# string. Formatters are used with syntax like ``{value:formatterName}`` or
# ``{value:formatterName(arg1,arg2)}``.
#
# Available formatters:
# - upper: Convert to uppercase
# - lower: Convert to lowercase
# - currency: Format as currency with locale support
# - date: Format dates with locale and format options
# - time: Format time with locale support
# - number: Format numbers with precision control
# - plural: Choose singular/plural form based on count
# - pad: Pad string with characters to a specified length
default_formatters: dict[str, Formatter] = {
    "upper": _upper,
    "lower": _lower,
    "currency": _currency,
    "date": _date,
    "time": _time,
    "number": _number,
    "plural": _plural,
    "pad": _pad,
}
