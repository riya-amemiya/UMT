import calendar
from datetime import datetime, timedelta

from .start_of import DateBoundaryUnit

__all__ = ["DateBoundaryUnit", "end_of"]


def end_of(date: datetime, unit: str) -> datetime:
    """
    Return a new datetime set to the end of the given unit.

    Week ends on Saturday at 23:59:59.999 (JavaScript day 6). Unknown units
    leave the date unchanged.

    Args:
        date: Base date
        unit: Boundary unit
            ("second"|"minute"|"hour"|"day"|"week"|"month"|"quarter"|"year")

    Returns:
        A new datetime at the end of the specified unit

    Example:
        >>> from datetime import datetime
        >>> end_of(datetime(2025, 4, 15, 10, 30, 0), "day")
        datetime.datetime(2025, 4, 15, 23, 59, 59, 999000)
        >>> end_of(datetime(2025, 4, 15), "month")
        datetime.datetime(2025, 4, 30, 23, 59, 59, 999000)
    """
    if unit == "second":
        return date.replace(microsecond=999000)
    if unit == "minute":
        return date.replace(second=59, microsecond=999000)
    if unit == "hour":
        return date.replace(minute=59, second=59, microsecond=999000)
    if unit == "day":
        return date.replace(hour=23, minute=59, second=59, microsecond=999000)
    if unit == "week":
        day_of_week = (date.weekday() + 1) % 7
        result = date + timedelta(days=6 - day_of_week)
        return result.replace(hour=23, minute=59, second=59, microsecond=999000)
    if unit == "month":
        last_day = calendar.monthrange(date.year, date.month)[1]
        return date.replace(
            day=last_day,
            hour=23,
            minute=59,
            second=59,
            microsecond=999000,
        )
    if unit == "quarter":
        quarter_end_month = ((date.month - 1) // 3) * 3 + 3
        last_day = calendar.monthrange(date.year, quarter_end_month)[1]
        return date.replace(
            month=quarter_end_month,
            day=last_day,
            hour=23,
            minute=59,
            second=59,
            microsecond=999000,
        )
    if unit == "year":
        return date.replace(
            month=12,
            day=31,
            hour=23,
            minute=59,
            second=59,
            microsecond=999000,
        )
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
