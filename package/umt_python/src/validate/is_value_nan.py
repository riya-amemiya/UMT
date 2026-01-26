import math


def is_value_nan(value: object, loose: bool = False) -> bool:
    """
    Determines if a value is NaN.

    Args:
        value: Value to check.
        loose: If True, includes string values in NaN check (default: False).

    Returns:
        True if the value is NaN, False otherwise.

    Example:
        >>> is_value_nan(1)
        False
        >>> is_value_nan("NaN")
        False
        >>> is_value_nan("NaN", True)
        True
        >>> is_value_nan(float("nan"))
        True
    """
    if loose:
        try:
            return math.isnan(float(value))  # type: ignore[arg-type]
        except (ValueError, TypeError):
            return False
    return isinstance(value, float) and math.isnan(value)
