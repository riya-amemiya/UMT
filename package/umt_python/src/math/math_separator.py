from ..validate import is_number
from .js_string_to_number import js_string_to_number
from .number_to_js_string import number_to_js_string


def math_separator(input_value: str | float) -> tuple[float, float]:
    """
    Separates a number at its highest place value.

    Args:
        input_value: Value to separate.

    Returns:
        Tuple of [primary value (highest place), remainder].

    Example:
        >>> math_separator(1250)
        (1000, 250)
        >>> math_separator(5)
        (5, 0)
    """
    if not is_number(input_value):
        return (0, 0)

    string_value = (
        input_value
        if isinstance(input_value, str)
        else number_to_js_string(input_value)
    )
    integer_part, _, fractional_part = string_value.partition(".")
    decimal_part = (
        js_string_to_number(f"0.{fractional_part}") if fractional_part else 0.0
    )

    number_of_digits = len(integer_part) - 1
    numerical_value = js_string_to_number(integer_part)

    if number_of_digits == 0:
        return (numerical_value, decimal_part)

    primary = 10**number_of_digits
    remainder = numerical_value - primary + decimal_part

    return (float(primary), remainder)
