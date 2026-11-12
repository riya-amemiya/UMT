from .number_to_js_string import number_to_js_string


def get_decimal_length(value: float) -> int:
    """
    Gets the number of decimal places in a number.

    Args:
        value: Number to check.

    Returns:
        Number of decimal places (0 for integers).

    Example:
        >>> get_decimal_length(1.23)
        2
        >>> get_decimal_length(100)
        0
        >>> get_decimal_length(1.0)
        0
    """
    if isinstance(value, int):
        return 0
    _, separator, decimal_part = number_to_js_string(value).partition(".")
    return len(decimal_part) if separator else 0
