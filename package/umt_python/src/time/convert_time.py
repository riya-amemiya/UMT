from typing import Literal, Union
from .normalize_time_unit import normalize_time_unit

TimeUnit = Literal["milliseconds", "seconds", "minutes", "hours"]
TimeUnitShort = Literal["ms", "s", "m", "h"]

CONVERSION_RATES = {
    "milliseconds": 1,
    "seconds": 1000,
    "minutes": 60000,
    "hours": 3600000,
}


def convert_time(
    value: Union[str, int, float],
    from_unit: Union[TimeUnit, TimeUnitShort],
    to_unit: Union[TimeUnit, TimeUnitShort],
) -> float:
    """
    Converts time between different units.

    Args:
        value: Value to convert (string or number)
        from_unit: Source time unit
        to_unit: Target time unit

    Returns:
        Converted value (number)

    Example:
        >>> convert_time(1, "hours", "minutes")
        60.0
        >>> convert_time("1000", "ms", "s")
        1.0
    """
    numeric_value = float(value) if isinstance(value, str) else value
    normalized_from_unit = normalize_time_unit(from_unit, "long")
    normalized_to_unit = normalize_time_unit(to_unit, "long")

    milliseconds = numeric_value * CONVERSION_RATES[normalized_from_unit]
    return milliseconds / CONVERSION_RATES[normalized_to_unit]
