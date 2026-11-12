from datetime import datetime

from .start_of import DateBoundaryUnit, start_of


def is_same(
    left: datetime,
    right: datetime,
    unit: DateBoundaryUnit | None = None,
) -> bool:
    """
    Return True when two dates are the same at the given unit granularity.

    Without a unit, compare exact timestamps. With a unit, truncate both
    dates with `start_of` before comparison. Week boundaries follow the
    Sunday-start week, matching `start_of`.

    Args:
        left: First date
        right: Second date
        unit: Optional boundary unit; omit for exact equality

    Returns:
        True when the dates match at the given granularity

    Example:
        >>> from datetime import datetime
        >>> is_same(datetime(2025, 4, 15, 1), datetime(2025, 4, 15, 23), "day")
        True
        >>> is_same(datetime(2025, 4, 15), datetime(2025, 5, 15), "month")
        False
        >>> is_same(datetime(2025, 1, 1), datetime(2025, 1, 1))
        True
    """
    if unit is None:
        return left.timestamp() == right.timestamp()
    return start_of(left, unit).timestamp() == start_of(right, unit).timestamp()
