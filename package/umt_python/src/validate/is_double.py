import math


def is_double(value: object, loose: bool = True) -> bool:
    """
    Determines if the value is a decimal number (has a fractional part).

    Args:
        value: Value to check.
        loose: Whether to include string representations of decimal numbers (default: True).

    Returns:
        True if the value is a decimal number, False otherwise.

    Example:
        >>> is_double(1.5)
        True
        >>> is_double("1.5")
        True
        >>> is_double(1)
        False
    """
    # Explicitly exclude non-numeric types that might be coerced or cause issues
    if isinstance(value, (list, dict, bool)) or value is None:
        return False

    if loose:
        try:
            num = float(value)  # type: ignore[arg-type]
        except (ValueError, TypeError):
            return False
    else:
        if not isinstance(value, (float, int)):
            return False
        num = float(value)

    return math.isfinite(num) and not num.is_integer()
