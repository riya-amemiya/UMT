import math


def is_number(value: object, loose: bool = True) -> bool:
    """
    Determines if the value represents a number.

    Args:
        value: Value to check.
        loose: Whether to include string representations of numbers (default: True).

    Returns:
        True if the value represents a number, False otherwise.

    Example:
        >>> is_number(0.1)
        True
        >>> is_number("0.1")
        True
        >>> is_number("0.1", False)
        False
    """
    if isinstance(value, (list, dict)):
        return False
    if isinstance(value, bool):
        return False
    if value is None:
        return False
    if loose:
        try:
            num = float(value)  # type: ignore[arg-type]
            return math.isfinite(num)
        except (ValueError, TypeError):
            return False
    return isinstance(value, (int, float)) and math.isfinite(value)
