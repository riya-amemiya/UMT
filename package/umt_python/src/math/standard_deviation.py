import statistics
from collections.abc import Sequence


def standard_deviation(values: Sequence[float]) -> float:
    """
    Calculates the standard deviation of a set of values.

    Args:
        values: Array of numeric values.

    Returns:
        The standard deviation.

    Example:
        >>> round(standard_deviation([1, 2, 3]), 6)
        0.816497
        >>> round(standard_deviation([10, 12, 23, 23, 16, 23, 21, 16]), 6)
        4.89898
    """
    if not values:
        return 0.0

    return statistics.pstdev(values)
