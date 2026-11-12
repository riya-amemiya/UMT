from datetime import datetime, timedelta, timezone


def day_of_week(
    year: int | None = None,
    mon: int | None = None,
    day: int | None = None,
    time_difference: int = 9,
) -> int:
    """
    Get the day of the week.

    Args:
        year: The year (optional, defaults to current year)
        mon: The month 1-12 (optional, defaults to current month)
        day: The day (optional, defaults to current day)
        time_difference: Time difference from UTC in hours (default: 9)

    Returns:
        A number representing the day of the week (0 = Sunday, 6 = Saturday)

    Example:
        >>> day_of_week(2000, 1, 1)  # Saturday
        6
    """
    target_tz = timezone(timedelta(hours=time_difference))
    now_time = datetime.now(target_tz)

    target_year = year if year is not None else now_time.year
    target_mon = mon if mon is not None else now_time.month
    target_day = day if day is not None else now_time.day

    target_date = datetime(target_year, target_mon, target_day, tzinfo=target_tz)
    return (target_date.weekday() + 1) % 7
