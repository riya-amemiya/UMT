def lerp(start: int | float, end: int | float, t: int | float) -> float:
    """
    Linearly interpolates between two values.

    Args:
        start: Value returned when t is 0.
        end: Value returned when t is 1.
        t: Interpolation factor; values outside 0..1 extrapolate.

    Returns:
        start + (end - start) * t.

    Example:
        >>> lerp(0, 10, 0.5)
        5.0
        >>> lerp(10, 20, 0.25)
        12.5
        >>> lerp(0, 10, 2)
        20
    """
    return start + (end - start) * t
