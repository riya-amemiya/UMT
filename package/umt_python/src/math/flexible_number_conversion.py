import re

from .js_string_to_number import JS_WHITESPACE, js_string_to_number

_LEADING_FLOAT_RE = re.compile(r"[+-]?(?:[0-9]+\.?[0-9]*|\.[0-9]+)(?:e[+-]?[0-9]+)?")


def flexible_number_conversion(value: object) -> float:
    """
    Flexible function to convert various inputs to numbers whenever possible.

    Args:
        value: Value to convert (any type).

    Returns:
        Converted number, or NaN if conversion is not possible.

    Example:
        >>> flexible_number_conversion(123)
        123
        >>> flexible_number_conversion("456")
        456.0
        >>> flexible_number_conversion("78.9")
        78.9
        >>> flexible_number_conversion("3.14e2")
        314.0
        >>> flexible_number_conversion("0xFF")
        255.0
        >>> flexible_number_conversion("42px")
        42.0
        >>> flexible_number_conversion("")
        0
        >>> math.isnan(flexible_number_conversion("not a number"))
        True
    """
    if value is None or value == "":
        return 0

    if isinstance(value, bool) or not isinstance(value, (str, int, float)):
        return float("nan")

    if not isinstance(value, str):
        return value

    string_value = value.strip(JS_WHITESPACE).lower()

    if string_value in ("infinity", "+infinity"):
        return float("inf")
    if string_value == "-infinity":
        return float("-inf")

    if string_value.startswith(("0x", "0o", "0b")):
        return js_string_to_number(string_value)

    match = _LEADING_FLOAT_RE.match(string_value)
    return float(match.group()) if match else float("nan")
