import math
from datetime import datetime

from ..consts.clock_consts import ONE_HOUR_MS, ONE_MINUTE_MS, ONE_SECOND_MS
from .ms_by_unit import ms_by_unit


def diff(left: datetime, right: datetime, unit: str) -> int:
    """
    Return the difference between two dates in the given unit, truncated toward
    zero.

    Calendar-aware for "M" (months) and "y" (years).

    Args:
        left: End date
        right: Start date
        unit: Unit of the result
            ("ms"|"s"|"m"|"h"|"d"|"w"|"M"|"y")

    Returns:
        The difference, truncated toward zero

    Example:
        >>> from datetime import datetime
        >>> diff(datetime(2025, 12, 31), datetime(2025, 1, 1), "d")
        364
        >>> diff(datetime(2026, 1, 1), datetime(2025, 1, 1), "y")
        1
    """
    ms = ms_by_unit.get(unit)
    if ms is not None:
        delta = left - right
        delta_ms = (
            delta.days * 86_400_000 + delta.seconds * 1000 + delta.microseconds // 1000
        )
        return math.trunc(delta_ms / ms)

    years_delta = left.year - right.year
    months_delta = left.month - right.month
    day_delta = left.day - right.day
    sub_day_delta = (
        left.hour * ONE_HOUR_MS
        + left.minute * ONE_MINUTE_MS
        + left.second * ONE_SECOND_MS
        + left.microsecond // 1000
    ) - (
        right.hour * ONE_HOUR_MS
        + right.minute * ONE_MINUTE_MS
        + right.second * ONE_SECOND_MS
        + right.microsecond // 1000
    )

    calendar_months = years_delta * 12 + months_delta
    if calendar_months > 0 and (
        day_delta < 0 or (day_delta == 0 and sub_day_delta < 0)
    ):
        calendar_months -= 1
    elif calendar_months < 0 and (
        day_delta > 0 or (day_delta == 0 and sub_day_delta > 0)
    ):
        calendar_months += 1

    if unit == "M":
        return calendar_months
    return math.trunc(calendar_months / 12)
