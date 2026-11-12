import math


def max_value(*numbers: float) -> float:
    """
    Returns the maximum value from the input numbers.

    Args:
        *numbers: Variable number of numbers.

    Returns:
        Maximum value.

    Example:
        >>> max_value(1, 2, 3)
        3
        >>> max_value(1, 2, 2, 3)
        3
    """
    if not numbers:
        return float("-inf")
    if any(math.isnan(number) for number in numbers):
        return float("nan")
    return max(numbers)
