from collections.abc import Sequence

from ...math.average import average
from ...math.deviation_value import deviation_value
from ...math.standard_deviation import standard_deviation


def deviation_value_simple(
    value: float,
    average_value: Sequence[float] | float,
    standard_deviation_value: float | None = None,
) -> float:
    """
    Calculate deviation score (T-score).

    Args:
        value: Input value.
        average_value: Average value or array of values to calculate average from.
        standard_deviation_value: Standard deviation (optional if average_value is array).

    Returns:
        Deviation score (50 is average, each standard deviation is worth 10 points).
        Returns 50 when standard deviation is 0 (all values are the same).

    Raises:
        TypeError: If standard deviation is missing when using a single average value.

    Example:
        >>> deviation_value_simple(60, 50, 10)
        60.0
        >>> deviation_value_simple(60, [40, 50, 60])
        62.24744871391589
    """
    # Handle sequence input (list, tuple, etc.), but not strings/bytes
    if isinstance(average_value, Sequence) and not isinstance(
        average_value, (str, bytes)
    ):
        avg = average(average_value)
        sd = standard_deviation(average_value)
        # When all values are the same, standard deviation is 0
        return 50 if sd == 0 else deviation_value(value, avg, sd)

    # Handle direct value input with standard deviation
    if standard_deviation_value is None:
        raise TypeError(
            "Standard deviation is required when using a single average value"
        )

    # Ensure average_value is treated as float for calculation
    # (though type hint says float, runtime could be int, which is fine)
    avg_val = float(average_value)  # type: ignore

    return (
        50
        if standard_deviation_value == 0
        else deviation_value(value, avg_val, standard_deviation_value)
    )
