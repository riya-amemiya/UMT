from collections.abc import Sequence

from src.math.average import average
from src.math.deviation_value import deviation_value
from src.math.standard_deviation import standard_deviation


def deviation_value_simple(
    value: float | int,
    average_value: float | int | Sequence[float | int],
    standard_deviation_value: float | int | None = None,
) -> float:
    """
    Calculate deviation score (T-score)

    Args:
        value: Input value
        average_value: Average value or sequence of values to calculate average from
        standard_deviation_value: Standard deviation (optional if average_value is sequence)

    Returns:
        Deviation score (50 is average, each standard deviation is worth 10 points)
        Returns 50 when standard deviation is 0 (all values are the same)

    Example:
        >>> deviation_value_simple(60, 50, 10)
        60.0
        >>> deviation_value_simple(60, [40, 50, 60])
        62.24744871391589
    """
    if isinstance(average_value, Sequence) and not isinstance(average_value, str):
        avg = average(list(average_value))
        sd = standard_deviation(list(average_value))
        return 50.0 if sd == 0 else deviation_value(value, avg, sd)

    if standard_deviation_value is None:
        raise TypeError(
            "Standard deviation is required when using a single average value"
        )

    return (
        50.0
        if standard_deviation_value == 0
        else deviation_value(value, average_value, standard_deviation_value)  # type: ignore
    )
