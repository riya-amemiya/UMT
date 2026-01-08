from ..validate import is_number


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

    string_value = str(input_value)
    parts = string_value.split(".")
    integer_part = parts[0]
    decimal_part = float(f"0.{parts[1]}") if len(parts) > 1 else 0.0

    number_of_digits = len(integer_part) - 1
    numerical_value = int(integer_part)

    if number_of_digits == 0:
        return (float(numerical_value), decimal_part)

    primary = 10**number_of_digits
    remainder = numerical_value - primary + decimal_part

    return (float(primary), remainder)
