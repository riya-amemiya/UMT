import math


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
    try:
        multiplier = float(10**precision)
    except OverflowError:
        multiplier = math.inf
    if multiplier == 0:
        return math.nan
    scaled = value * multiplier
    if not math.isfinite(scaled):
        return scaled / multiplier
    lower = math.floor(scaled)
    return (lower + 1 if scaled - lower >= 0.5 else lower) / multiplier
