from datetime import datetime

from .start_of import DateBoundaryUnit, start_of


def is_after(
    date: datetime,
    compare: datetime,
    unit: DateBoundaryUnit | None = None,
) -> bool:
    """
    Return True when `date` is strictly after `compare` at the given granularity.

    Without a unit, compare exact timestamps. With a unit, truncate both
    dates with `start_of` before comparison. Week boundaries follow the
    Sunday-start week, matching `start_of`.

    Args:
        date: Date to test
        compare: Date to compare against
        unit: Optional boundary unit; omit for exact comparison

    Returns:
        True when `date` is after `compare`

    Example:
        >>> from datetime import datetime
        >>> is_after(datetime(2025, 4, 16), datetime(2025, 4, 15))
        True
        >>> is_after(datetime(2025, 4, 15, 23), datetime(2025, 4, 15, 1), "day")
        False
        >>> is_after(datetime(2025, 5, 1), datetime(2025, 4, 30), "month")
        True
    """
    if unit is None:
        return date.timestamp() > compare.timestamp()
    return start_of(date, unit).timestamp() > start_of(compare, unit).timestamp()
