from datetime import datetime

from .start_of import start_of


def week_of_year(date: datetime) -> int:
    """
    Return the Sunday-start week number of the date's year.

    Week 1 is the Sunday-start week that contains January 1.

    Args:
        date: Date to inspect

    Returns:
        Week number starting at 1

    Example:
        >>> from datetime import datetime
        >>> week_of_year(datetime(2025, 1, 1))
        1
        >>> week_of_year(datetime(2025, 1, 5))
        2
    """
    week_start = start_of(date, "week")
    year_start = date.replace(month=1, day=1, hour=0, minute=0, second=0, microsecond=0)
    year_start_week = start_of(year_start, "week")
    days = round((week_start - year_start_week).total_seconds() / 86_400)
    return days // 7 + 1
