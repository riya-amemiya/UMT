import math
import statistics
from collections.abc import Sequence

from .variance import variance


def standard_deviation(values: Sequence[float], sample: bool = False) -> float:
    """
    Calculates the standard deviation of a set of values.

    Args:
        values: Array of numeric values.
        sample: If True, uses the sample variance (n - 1) instead of the
            population variance (n).

    Returns:
        The standard deviation, 0 for an empty array, or NaN for a sample of
        a single value.

    Example:
        >>> round(standard_deviation([1, 2, 3]), 6)
        0.816497
        >>> standard_deviation([1, 2, 3], True)
        1.0
        >>> round(standard_deviation([10, 12, 23, 23, 16, 23, 21, 16]), 6)
        4.89898
    """
    if not values:
        return 0.0

    if sample:
        return math.sqrt(variance(values, True))

    return statistics.pstdev(values)
