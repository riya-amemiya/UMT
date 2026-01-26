def value_swap(x: float, y: float) -> tuple[float, float]:
    """
    Swaps two numbers to ensure x < y.

    Args:
        x: First number.
        y: Second number.

    Returns:
        Tuple with numbers in ascending order.

    Example:
        >>> value_swap(2, 1)
        (1, 2)
        >>> value_swap(1, 2)
        (1, 2)
    """
    return (x, y) if x < y else (y, x)
