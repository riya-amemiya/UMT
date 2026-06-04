from datetime import datetime, timedelta
from typing import Literal

DateBoundaryUnit = Literal[
    "second",
    "minute",
    "hour",
    "day",
    "week",
    "month",
    "quarter",
    "year",
]


def start_of(date: datetime, unit: str) -> datetime:
    """
    Return a new datetime set to the start of the given unit.

    Week start is Sunday (JavaScript day 0). Unknown units leave the date
    unchanged.

    Args:
        date: Base date
        unit: Boundary unit
            ("second"|"minute"|"hour"|"day"|"week"|"month"|"quarter"|"year")

    Returns:
        A new datetime at the start of the specified unit

    Example:
        >>> from datetime import datetime
        >>> start_of(datetime(2025, 4, 15, 10, 30, 0), "day")
        datetime.datetime(2025, 4, 15, 0, 0)
        >>> start_of(datetime(2025, 4, 15), "month")
        datetime.datetime(2025, 4, 1, 0, 0)
    """
    if unit == "second":
        return date.replace(microsecond=0)
    if unit == "minute":
        return date.replace(second=0, microsecond=0)
    if unit == "hour":
        return date.replace(minute=0, second=0, microsecond=0)
    if unit == "day":
        return date.replace(hour=0, minute=0, second=0, microsecond=0)
    if unit == "week":
        day_of_week = (date.weekday() + 1) % 7
        result = date - timedelta(days=day_of_week)
        return result.replace(hour=0, minute=0, second=0, microsecond=0)
    if unit == "month":
        return date.replace(day=1, hour=0, minute=0, second=0, microsecond=0)
    if unit == "quarter":
        quarter_start_month = ((date.month - 1) // 3) * 3 + 1
        return date.replace(
            month=quarter_start_month,
            day=1,
            hour=0,
            minute=0,
            second=0,
            microsecond=0,
        )
    if unit == "year":
        return date.replace(month=1, day=1, hour=0, minute=0, second=0, microsecond=0)
    return datetime(
        date.year,
        date.month,
        date.day,
        date.hour,
        date.minute,
        date.second,
        date.microsecond,
        tzinfo=date.tzinfo,
        fold=date.fold,
    )
