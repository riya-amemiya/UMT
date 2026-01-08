def to_base_n(value: int, radix: int = 2) -> str:
    """
    Converts a number to a string representation in the specified base.

    Args:
        value: The number to convert.
        radix: The base to convert to (2-36).

    Returns:
        String representation of the number in the specified base.

    Example:
        >>> to_base_n(10)
        '1010'
        >>> to_base_n(15, 16)
        'f'
        >>> to_base_n(255, 16)
        'ff'
    """
    if radix == 2:
        return bin(value)[2:]
    elif radix == 8:
        return oct(value)[2:]
    elif radix == 16:
        return hex(value)[2:]
    else:
        if value == 0:
            return "0"
        digits = "0123456789abcdefghijklmnopqrstuvwxyz"
        result = ""
        is_negative = value < 0
        value = abs(value)
        while value:
            result = digits[value % radix] + result
            value //= radix
        return "-" + result if is_negative else result
