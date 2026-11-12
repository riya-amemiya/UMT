from collections.abc import Sequence

from .addition import addition
from .average import average
from .division import division
from .multiplication import multiplication
from .subtraction import subtraction


def variance(values: Sequence[int | float], sample: bool = False) -> float:
    """
    Calculates the variance of a set of values.

    Args:
        values: Array of numeric values.
        sample: If True, divides by n - 1 (sample variance) instead of n
            (population variance).

    Returns:
        The variance, 0 for an empty array, or NaN for a sample of a single
        value.

    Example:
        >>> variance([1, 2, 3])
        0.6666666666666666
        >>> variance([1, 2, 3], True)
        1.0
        >>> variance([10, 12, 23, 23, 16, 23, 21, 16])
        24.0
    """
    if len(values) == 0:
        return 0.0
    if sample and len(values) == 1:
        return float("nan")

    avg = average(values)
    sum_square_diffs: float = 0
    for value in values:
        diff = subtraction(value, avg)
        sum_square_diffs = addition(sum_square_diffs, multiplication(diff, diff))

    return float(division(sum_square_diffs, len(values) - 1 if sample else len(values)))
