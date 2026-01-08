def round_of(value: float, precision: int = 0) -> float:
    """
    Rounds a number to specified decimal places.

    Args:
        value: Number to round.
        precision: Number of decimal places (default: 0).

    Returns:
        Rounded number.

    Example:
        >>> round_of(1.234, 2)
        1.23
        >>> round_of(1.235, 2)
        1.24
        >>> round_of(-1.234, 2)
        -1.23
    """
    multiplier = 10**precision
    return round(value * multiplier) / multiplier
