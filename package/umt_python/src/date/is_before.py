from datetime import datetime

from .start_of import DateBoundaryUnit, start_of


def is_before(
    date: datetime,
    compare: datetime,
    unit: DateBoundaryUnit | None = None,
) -> bool:
    """
    Return True when `date` is strictly before `compare` at the given granularity.

    Without a unit, compare exact timestamps. With a unit, truncate both
    dates with `start_of` before comparison. Week boundaries follow the
    Sunday-start week, matching `start_of`.

    Args:
        date: Date to test
        compare: Date to compare against
        unit: Optional boundary unit; omit for exact comparison

    Returns:
        True when `date` is before `compare`

    Example:
        >>> from datetime import datetime
        >>> is_before(datetime(2025, 4, 15), datetime(2025, 4, 16))
        True
        >>> is_before(datetime(2025, 4, 15, 1), datetime(2025, 4, 15, 23), "day")
        False
        >>> is_before(datetime(2025, 4, 30), datetime(2025, 5, 1), "month")
        True
    """
    if unit is None:
        return date.timestamp() < compare.timestamp()
    return start_of(date, unit).timestamp() < start_of(compare, unit).timestamp()
