import math


def is_double(x: object, loose: bool = True) -> bool:
    """
    Determines if the value is a decimal number.

    Args:
        x: Value to check.
        loose: Whether to include string representations of decimal numbers.

    Returns:
        True if the value is a decimal number, False otherwise.

    Example:
        >>> is_double(0.1)
        True
        >>> is_double("0.1")
        True
        >>> is_double("0.1", False)
        False
    """
    if loose:
        try:
            num = float(x)  # type: ignore[arg-type]
            return math.isfinite(num) and not float(num).is_integer()
        except (ValueError, TypeError):
            return False
    return isinstance(x, float) and math.isfinite(x) and not x.is_integer()
