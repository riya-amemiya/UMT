from datetime import datetime, timezone

from .unix_time_unit import UnixTimeUnit


def from_unix(value: int | float, unit: UnixTimeUnit = "s") -> datetime:
    """
    Create a datetime from a Unix timestamp.

    Default unit is seconds. The result is a naive local datetime whose
    epoch offset matches JavaScript Date.getTime().

    Args:
        value: Timestamp value
        unit: "s" for seconds or "ms" for milliseconds

    Returns:
        datetime for the given epoch offset

    Example:
        >>> from_unix(0).timestamp()
        0.0
    """
    milliseconds = value * 1000 if unit == "s" else value
    aware = datetime.fromtimestamp(milliseconds / 1000, tz=timezone.utc)
    return aware.astimezone().replace(tzinfo=None)
