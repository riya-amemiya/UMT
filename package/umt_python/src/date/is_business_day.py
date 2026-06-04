from datetime import datetime

from .is_same_day import is_same_day
from .is_weekend import is_weekend


def is_business_day(date: datetime, holidays: list[datetime] | None = None) -> bool:
    """
    Return True when the given date is a weekday and not in the holidays list.

    Args:
        date: The date to check
        holidays: Optional list of holiday dates (defaults to empty)

    Returns:
        True when weekday and not a listed holiday

    Example:
        >>> from datetime import datetime
        >>> is_business_day(datetime(2025, 4, 21))  # Monday
        True
        >>> is_business_day(datetime(2025, 4, 21), [datetime(2025, 4, 21)])
        False
    """
    if holidays is None:
        holidays = []
    if is_weekend(date):
        return False
    return not any(is_same_day(holiday, date) for holiday in holidays)
