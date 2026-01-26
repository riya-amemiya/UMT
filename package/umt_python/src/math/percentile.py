import math
from collections.abc import Sequence


def percentile(array: Sequence[float], pct: float) -> float:
    """
    Calculate the nth percentile of values in an array.

    Args:
        array: Array of numbers.
        pct: Percentile value (0-100).

    Returns:
        The percentile value.

    Raises:
        ValueError: If percentile is not between 0 and 100.

    Example:
        >>> percentile([1, 2, 3, 4, 5], 50)
        3
        >>> percentile([1, 2, 3, 4, 5], 25)
        2
        >>> percentile([1, 2, 3, 4, 5], 75)
        4
    """
    if len(array) == 0:
        return float("nan")

    if pct < 0 or pct > 100:
        raise ValueError("Percentile must be between 0 and 100")

    sorted_array = sorted(array)
    index = (pct / 100) * (len(sorted_array) - 1)
    lower_index = math.floor(index)
    upper_index = math.ceil(index)

    if lower_index == upper_index:
        return sorted_array[lower_index]

    lower_value = sorted_array[lower_index]
    upper_value = sorted_array[upper_index]
    weight = index - lower_index

    return lower_value + (upper_value - lower_value) * weight
