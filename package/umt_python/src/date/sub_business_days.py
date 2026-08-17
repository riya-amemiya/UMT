from datetime import datetime

from .add_business_days import add_business_days


def sub_business_days(
    date: datetime,
    amount: int | float,
    holidays: list[datetime] | None = None,
) -> datetime:
    """
    Subtract business days from a date.

    Equivalent to add_business_days(date, -amount, holidays).

    Args:
        date: Base date
        amount: Business days to subtract
        holidays: Optional holiday dates

    Returns:
        A new datetime after walking backward `amount` business days

    Example:
        >>> from datetime import datetime
        >>> sub_business_days(datetime(2025, 4, 21, 9, 30), 1)
        datetime.datetime(2025, 4, 18, 9, 30)
    """
    return add_business_days(date, -amount, holidays)
