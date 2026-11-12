import math


def clamp(value: float, min_val: float, max_val: float) -> float:
    """
    Clamps a number between a minimum and maximum value.

    Args:
        value: The number to clamp.
        min_val: The minimum bound.
        max_val: The maximum bound.

    Returns:
        The clamped number.

    Example:
        >>> clamp(5, 0, 10)
        5
        >>> clamp(-3, 0, 10)
        0
        >>> clamp(15, 0, 10)
        10
    """
    if math.isnan(value) or math.isnan(min_val) or math.isnan(max_val):
        return float("nan")
    return min(max(value, min_val), max_val)
