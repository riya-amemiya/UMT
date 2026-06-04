import math
from collections.abc import Mapping

from ..validate.is_number import is_number
from .multiplication import multiplication
from .number_to_js_string import number_to_js_string


def convert_currency(
    input_string: str,
    conversion_rates: Mapping[str, float | str] | None = None,
) -> str:
    """
    Converts currency amounts in a string using currency symbols.

    Args:
        input_string: String containing a currency amount to convert.
        conversion_rates: Mapping of currency symbols to conversion rates.

    Returns:
        Converted currency amount as a string, or the original string if
        conversion is not possible.

    Example:
        >>> convert_currency("¥100", {"¥": 0.01})
        '1'
        >>> convert_currency("$50", {"$": 1.2})
        '60'
        >>> convert_currency("€200", {"€": 1.1})
        '220'
    """
    if not conversion_rates:
        return input_string

    for currency_symbol in conversion_rates:
        if input_string.startswith(currency_symbol):
            amount_string = input_string[len(currency_symbol) :]
            rate = conversion_rates[currency_symbol]

            if is_number(rate):
                amount = _to_number(amount_string)
                converted_amount = multiplication(amount, float(rate))
                if math.isnan(converted_amount):
                    return input_string
                return number_to_js_string(converted_amount)
    return input_string


def _to_number(value: str) -> float:
    """
    Mimics JavaScript's ``Number(value)`` coercion for a string.

    Returns NaN (``float('nan')``) when the string is not a valid number,
    matching JavaScript's behavior instead of raising an error.
    """
    stripped = value.strip()
    if stripped == "":
        return 0.0
    try:
        return float(stripped)
    except ValueError:
        return float("nan")
