import math
from collections.abc import Sequence

from .average import average
from .multiplication import multiplication
from .subtraction import subtraction


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
    avg = average(values)

    # Calculate the squared differences from the mean
    square_diffs = []
    for value in values:
        diff = subtraction(value, avg)
        square_diffs.append(multiplication(diff, diff))

    # Calculate the mean of the squared differences
    avg_square_diff = average(square_diffs)

    # Return the square root of the mean squared differences
    return math.sqrt(avg_square_diff)
