from collections.abc import Sequence

from .multiplication import multiplication
from .percentile import percentile


def quantile(values: Sequence[float], q: int | float) -> float:
    """
    Calculates the q-th quantile of values using linear interpolation.

    Args:
        values: Array of numbers.
        q: Quantile (0-1), equivalent to percentile(values, q * 100).

    Returns:
        The quantile value, or NaN for an empty array.

    Raises:
        ValueError: If q is not between 0 and 1.

    Example:
        >>> quantile([1, 2, 3, 4, 5], 0.5)
        3
        >>> quantile([1, 2, 3, 4, 5], 0.25)
        2
        >>> quantile([1, 2, 3, 4], 0.5)
        2.5
    """
    return percentile(values, multiplication(q, 100))
