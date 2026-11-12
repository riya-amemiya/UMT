import calendar
from datetime import datetime


def days_in_month(date: datetime) -> int:
    """
    Return the number of days in the month of the given date.

    Leap years follow the Gregorian calendar (February has 29 days).

    Args:
        date: Any date within the month

    Returns:
        Number of days in that month (28-31)

    Example:
        >>> from datetime import datetime
        >>> days_in_month(datetime(2025, 1, 15))
        31
        >>> days_in_month(datetime(2024, 2, 1))
        29
        >>> days_in_month(datetime(2025, 2, 1))
        28
    """
    return calendar.monthrange(date.year, date.month)[1]
