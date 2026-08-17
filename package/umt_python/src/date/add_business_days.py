from datetime import datetime, timedelta

from .is_business_day import is_business_day


def add_business_days(
    date: datetime,
    amount: int | float,
    holidays: list[datetime] | None = None,
) -> datetime:
    """
    Add business days to a date, skipping weekends and holidays.

    The starting date is not counted. A zero amount returns a clone.

    Args:
        date: Base date
        amount: Business days to add (may be negative)
        holidays: Optional holiday dates

    Returns:
        A new datetime after walking `amount` business days

    Example:
        >>> from datetime import datetime
        >>> add_business_days(datetime(2025, 4, 18, 9, 30), 1)
        datetime.datetime(2025, 4, 21, 9, 30)
    """
    if holidays is None:
        holidays = []
    result = date
    if amount == 0:
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
    step = 1 if amount > 0 else -1
    remaining = abs(amount)
    while remaining > 0:
        result = result + timedelta(days=step)
        if is_business_day(result, holidays):
            remaining -= 1
    return result
