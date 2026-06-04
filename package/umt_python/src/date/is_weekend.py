from datetime import datetime


def is_weekend(date: datetime) -> bool:
    """
    Return True when the given date is Saturday or Sunday.

    Args:
        date: The date to check

    Returns:
        True if Saturday or Sunday, False otherwise

    Example:
        >>> from datetime import datetime
        >>> is_weekend(datetime(2025, 4, 19))  # Saturday
        True
        >>> is_weekend(datetime(2025, 4, 21))  # Monday
        False
    """
    day_of_week = (date.weekday() + 1) % 7
    return day_of_week in {0, 6}
