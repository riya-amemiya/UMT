from datetime import datetime, timedelta

from .is_business_day import is_business_day
from .start_of import start_of


def business_days_between(
    start: datetime,
    end: datetime,
    holidays: list[datetime] | None = None,
) -> int:
    """
    Count business days from `start` to `end`.

    Business days follow `is_business_day` (weekdays that are not listed
    holidays). Days are compared by calendar date; the time of day is
    ignored. The start day is excluded and the end day is included, so the
    result is the inverse of `add_business_days`: when `end` is a business
    day, `add_business_days(start, business_days_between(start, end, holidays),
    holidays)` lands on the calendar day of `end`. When `end` is before
    `start`, the result is negative (the start day is still excluded and the
    end day is still included).

    Args:
        start: Start date (excluded)
        end: End date (included)
        holidays: Optional holiday dates

    Returns:
        Signed number of business days between the dates

    Example:
        >>> from datetime import datetime
        >>> business_days_between(datetime(2025, 4, 18), datetime(2025, 4, 21))
        1
        >>> business_days_between(datetime(2025, 4, 21), datetime(2025, 4, 25))
        4
        >>> business_days_between(datetime(2025, 4, 21), datetime(2025, 4, 18))
        -1
    """
    if holidays is None:
        holidays = []
    from_day = start_of(start, "day")
    days = (end.date() - start.date()).days
    step = -1 if days < 0 else 1
    return sum(
        step
        for index in range(abs(days))
        if is_business_day(from_day + timedelta(days=step * (index + 1)), holidays)
    )
