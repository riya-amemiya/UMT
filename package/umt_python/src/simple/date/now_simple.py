from datetime import datetime

from src.date.date_now import date_now


def now_simple(time_difference: int | str = 9) -> datetime:
    """
    Get the current date and time with a specified UTC offset.

    Accepts both int and str for time_difference, matching the
    flexible input handling of the TypeScript version.

    Args:
        time_difference: Hours offset from UTC (default: 9 for JST).
            Accepts int or numeric string.

    Returns:
        Current datetime adjusted for the specified UTC offset.

    Example:
        >>> now_simple()       # Current time in JST (UTC+9)
        >>> now_simple(0)      # Current time in UTC
        >>> now_simple("9")   # Current time in JST via string
    """
    if isinstance(time_difference, str):
        try:
            return date_now(int(time_difference))
        except ValueError:
            return date_now()
    return date_now(time_difference)
