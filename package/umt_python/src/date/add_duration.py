import calendar
from datetime import datetime, timedelta

from .ms_by_unit import ms_by_unit


def add_duration(date: datetime, amount: int | float, unit: str) -> datetime:
    """
    Add a duration to a date and return a new datetime.

    Calendar-aware for "M" (months) and "y" (years), so end-of-month is
    preserved (e.g. Jan 31 + 1 month = Feb 28/29).

    Args:
        date: Base date
        amount: Amount to add (can be negative)
        unit: Unit of the amount
            ("ms"|"s"|"m"|"h"|"d"|"w"|"M"|"y")

    Returns:
        A new datetime with the duration added

    Example:
        >>> from datetime import datetime
        >>> add_duration(datetime(2025, 1, 31), 1, "M")
        datetime.datetime(2025, 2, 28, 0, 0)
        >>> add_duration(datetime(2025, 1, 1), 7, "d")
        datetime.datetime(2025, 1, 8, 0, 0)
    """
    ms = ms_by_unit.get(unit)
    if ms is not None:
        return date + timedelta(milliseconds=amount * ms)

    if unit == "M":
        target_month_index = (date.month - 1) + int(amount)
        target_year = date.year + target_month_index // 12
        target_month = target_month_index % 12 + 1
        last_day = calendar.monthrange(target_year, target_month)[1]
        return date.replace(
            year=target_year,
            month=target_month,
            day=min(date.day, last_day),
        )

    target_year = date.year + int(amount)
    target_month = date.month
    last_day = calendar.monthrange(target_year, target_month)[1]
    return date.replace(
        year=target_year,
        month=target_month,
        day=min(date.day, last_day),
    )
