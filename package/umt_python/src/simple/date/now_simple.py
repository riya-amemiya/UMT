import math
from datetime import datetime

from src.date.date_now import date_now
from src.math.js_string_to_number import js_string_to_number


def now_simple(time_difference: int | str = 9) -> datetime:
    """
    Get the current date and time with a specified UTC offset.

    Accepts both int and str for time_difference, matching the
    flexible input handling of the TypeScript version.

    Args:
        time_difference: Hours offset from UTC (default: 9 for JST).
            Accepts int or numeric string. Strings follow JavaScript number
            parsing; non-numeric strings fall back to the default offset.

    Returns:
        Current datetime adjusted for the specified UTC offset.

    Example:
        >>> now_simple()       # Current time in JST (UTC+9)
        >>> now_simple(0)      # Current time in UTC
        >>> now_simple("9")   # Current time in JST via string
    """
    if isinstance(time_difference, str):
        parsed = js_string_to_number(time_difference)
        if not math.isfinite(parsed):
            return date_now()
        return date_now(int(parsed) if parsed.is_integer() else parsed)  # pyright: ignore[reportArgumentType]
    return date_now(time_difference)
