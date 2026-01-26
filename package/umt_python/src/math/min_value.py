def min_value(*numbers: float) -> float:
    """
    Returns the minimum value from the input numbers.

    Args:
        *numbers: Variable number of numbers.

    Returns:
        Minimum value.

    Example:
        >>> min_value(1, 2, 3)
        1
        >>> min_value(1, 1, 2, 3)
        1
    """
    return min(set(numbers))
