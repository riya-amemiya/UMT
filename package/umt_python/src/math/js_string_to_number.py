import re

_DECIMAL_LITERAL_RE = re.compile(
    r"[+-]?(?:Infinity|(?:[0-9]+\.?[0-9]*|\.[0-9]+)(?:[eE][+-]?[0-9]+)?)"
)
_RADIX_LITERAL_RE = re.compile(r"0(?:[xX][0-9a-fA-F]+|[oO][0-7]+|[bB][01]+)")

JS_WHITESPACE = (
    "\t\n\v\f\r \u00a0\u1680\u2000\u2001\u2002\u2003\u2004\u2005\u2006"
    "\u2007\u2008\u2009\u200a\u2028\u2029\u202f\u205f\u3000\ufeff"
)


def js_string_to_number(value: str) -> float:
    """
    Converts a string to a number following JavaScript's ``Number(string)`` rules.

    Surrounding JavaScript whitespace is ignored, the empty string converts to ``0``,
    decimal literals accept an optional sign and exponent, ``Infinity`` is
    recognized with an optional sign, and unsigned ``0x`` / ``0o`` / ``0b``
    literals are parsed in their radix. Everything else, including Python-only
    spellings such as ``inf``, ``nan`` and ``1_000``, converts to ``NaN``.

    Args:
        value: String to convert.

    Returns:
        The JavaScript-equivalent numeric value of the string.

    Example:
        >>> js_string_to_number("12.5")
        12.5
        >>> js_string_to_number("0x10")
        16.0
        >>> js_string_to_number("")
        0.0
        >>> math.isnan(js_string_to_number("inf"))
        True
    """
    stripped = value.strip(JS_WHITESPACE)
    if stripped == "":
        return 0.0
    if _DECIMAL_LITERAL_RE.fullmatch(stripped):
        return float(stripped)
    if _RADIX_LITERAL_RE.fullmatch(stripped):
        try:
            return float(int(stripped, 0))
        except OverflowError:
            return float("inf")
    return float("nan")
