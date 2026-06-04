import math


def number_to_js_string(value: float) -> str:
    """
    Converts a number to its string form following JavaScript's
    ``Number.prototype.toString`` / ``String(number)`` rules.

    JavaScript prints integral floating point numbers without a trailing
    ``.0`` (``4`` not ``4.0``), prints ``Infinity`` / ``-Infinity`` / ``NaN``
    for the corresponding special values, uses ``0`` for negative zero, and
    switches to exponential notation only for very large (``>= 1e21``) or very
    small (``< 1e-6``) magnitudes.

    Args:
        value: Number to convert.

    Returns:
        The JavaScript-equivalent string representation of the number.

    Example:
        >>> number_to_js_string(4.0)
        '4'
        >>> number_to_js_string(0.3)
        '0.3'
        >>> number_to_js_string(float("inf"))
        'Infinity'
        >>> number_to_js_string(-0.0)
        '0'
    """
    if math.isnan(value):
        return "NaN"
    if math.isinf(value):
        return "Infinity" if value > 0 else "-Infinity"
    if value == 0:
        return "0"

    negative = value < 0
    magnitude = -value if negative else value

    if magnitude >= 1e21 or magnitude < 1e-6:
        body = _to_exponential(magnitude)
    else:
        body = _to_fixed_shortest(magnitude)

    return f"-{body}" if negative else body


def _to_fixed_shortest(magnitude: float) -> str:
    """Renders a positive, non-exponential number using its shortest form."""
    text = repr(magnitude)
    if "e" in text or "E" in text:
        # Python may still use exponential form; expand it back to plain digits.
        text = f"{magnitude:.17f}".rstrip("0").rstrip(".")
        # Re-shorten using repr round-trip safety.
        if float(text) != magnitude:
            text = repr(magnitude)
    if text.endswith(".0"):
        return text[:-2]
    return text


def _to_exponential(magnitude: float) -> str:
    """Renders a positive number using JavaScript-style exponential notation."""
    text = repr(magnitude)
    if "e" not in text and "E" not in text:
        text = f"{magnitude:e}"
    mantissa, _, exponent = text.lower().partition("e")
    if "." in mantissa:
        mantissa = mantissa.rstrip("0").rstrip(".")
    exp_value = int(exponent)
    sign = "+" if exp_value >= 0 else "-"
    return f"{mantissa}e{sign}{abs(exp_value)}"
