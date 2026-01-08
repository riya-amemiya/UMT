from collections.abc import Sequence

from .addition import addition
from .division import division


def average(numbers: Sequence[float]) -> float:
    """
    Calculates the arithmetic mean of an array of numbers.

    Args:
        numbers: Array of numbers to average.

    Returns:
        The arithmetic mean, returns 0 for empty array.

    Example:
        >>> average([1, 2, 3])
        2
        >>> average([10, 20])
        15
        >>> average([])
        0
    """
    if len(numbers) == 0:
        return 0

    total = 0.0
    for num in numbers:
        total = addition(total, num)
    avg = division(total, len(numbers))
    return avg
