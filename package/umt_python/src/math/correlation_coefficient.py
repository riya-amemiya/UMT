import math

from .average import average


def correlation_coefficient(x: list[float], y: list[float]) -> float:
    """
    Calculate the Pearson correlation coefficient between two arrays.

    Args:
        x: First array of numbers.
        y: Second array of numbers.

    Returns:
        Correlation coefficient (-1 to 1).

    Raises:
        ValueError: If arrays have different lengths.

    Example:
        >>> correlation_coefficient([1, 2, 3, 4, 5], [2, 4, 6, 8, 10])
        1.0
        >>> correlation_coefficient([1, 2, 3, 4, 5], [5, 4, 3, 2, 1])
        -1.0
    """
    if len(x) != len(y):
        raise ValueError("Arrays must have the same length")

    if len(x) == 0:
        return float("nan")

    if len(x) == 1:
        return float("nan")

    mean_x = average(x)
    mean_y = average(y)

    numerator = 0.0
    sum_squared_x = 0.0
    sum_squared_y = 0.0

    for index, element in enumerate(x):
        delta_x = element - mean_x
        delta_y = y[index] - mean_y

        numerator += delta_x * delta_y
        sum_squared_x += delta_x * delta_x
        sum_squared_y += delta_y * delta_y

    denominator = math.sqrt(sum_squared_x * sum_squared_y)

    if denominator == 0:
        return float("nan")

    return numerator / denominator
