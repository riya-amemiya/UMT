from datetime import datetime


def is_same_day(left: datetime, right: datetime) -> bool:
    """
    Return True when two dates represent the same calendar day in local time.

    Args:
        left: First date
        right: Second date

    Returns:
        True if same year/month/day in local time

    Example:
        >>> from datetime import datetime
        >>> is_same_day(datetime(2025, 4, 15, 1, 0), datetime(2025, 4, 15, 23, 0))
        True
        >>> is_same_day(datetime(2025, 4, 15), datetime(2025, 4, 16))
        False
    """
    return (
        left.year == right.year and left.month == right.month and left.day == right.day
    )
