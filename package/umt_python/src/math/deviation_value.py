import math


def deviation_value(
    value: float,
    average_value: float,
    standard_deviation_value: float,
) -> float:
    """
    Calculate standard score (deviation value).

    Args:
        value: Current value.
        average_value: Mean value.
        standard_deviation_value: Standard deviation.

    Returns:
        Standard score (where 50 is average, each standard deviation is worth 10 points).

    Example:
        >>> deviation_value(10, 5, 2)
        75.0
    """
    difference = value - average_value
    if standard_deviation_value == 0:
        if difference == 0 or math.isnan(difference):
            return float("nan")
        return math.copysign(float("inf"), difference) * math.copysign(
            1, standard_deviation_value
        )
    return (difference / standard_deviation_value) * 10 + 50
