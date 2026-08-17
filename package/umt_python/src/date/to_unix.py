import math
from datetime import datetime

from .unix_time_unit import UnixTimeUnit


def to_unix(date: datetime, unit: UnixTimeUnit = "s") -> float:
    """
    Convert a datetime to a Unix timestamp.

    Default unit is seconds, floored to an integer.

    Args:
        date: Date to convert
        unit: "s" for seconds or "ms" for milliseconds

    Returns:
        Unix timestamp in the requested unit

    Example:
        >>> from datetime import datetime, timezone
        >>> to_unix(datetime(1970, 1, 1, tzinfo=timezone.utc))
        0.0
    """
    milliseconds = date.timestamp() * 1000
    return math.floor(milliseconds / 1000) if unit == "s" else milliseconds
