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
    string_value = str(value)
    if "." in string_value:
        decimal_part = string_value.split(".")[1]
        if decimal_part and len(decimal_part) > 0:
            return len(decimal_part)
    return 0
