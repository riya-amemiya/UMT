import math


def to_percentage(value: float | int, total: float | int, decimals: int = 2) -> float:
    """
    Calculates the percentage of a value relative to a total.

    Returns 0 when the total is 0 to avoid division by zero.

    Args:
        value: The partial value.
        total: The total value.
        decimals: The number of decimal places (default 2).

    Returns:
        The percentage value.

    Example:
        >>> to_percentage(25, 100)
        25.0
        >>> to_percentage(1, 3)
        33.33
        >>> to_percentage(1, 3, 0)
        33.0
        >>> to_percentage(0, 0)
        0.0
        >>> to_percentage(50, 200, 1)
        25.0
    """
    if total == 0:
        return 0.0

    factor = 10**decimals
    # Mimic JS Math.round(x) behavior: floor(x + 0.5)
    return math.floor((value / total) * 100 * factor + 0.5) / factor
