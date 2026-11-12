def inverse_lerp(start: int | float, end: int | float, value: int | float) -> float:
    """
    Calculates the interpolation factor of a value between two values.

    Args:
        start: Value that maps to 0.
        end: Value that maps to 1.
        value: Value to locate; values outside the range give factors outside
            0..1.

    Returns:
        (value - start) / (end - start), or 0 when start equals end.

    Example:
        >>> inverse_lerp(0, 10, 5)
        0.5
        >>> inverse_lerp(10, 20, 12.5)
        0.25
        >>> inverse_lerp(5, 5, 7)
        0.0
    """
    if start == end:
        return 0.0
    return (value - start) / (end - start)
