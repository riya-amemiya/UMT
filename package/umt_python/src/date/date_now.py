from datetime import datetime, timezone, timedelta


def date_now(time_difference: int = 9) -> datetime:
    """
    Get the current time with a specified UTC offset, regardless of the local timezone.

    Args:
        time_difference: Hours offset from UTC (default: 9 for Japan Standard Time)

    Returns:
        Current date and time adjusted for the specified UTC offset

    Example:
        >>> date_now()  # Returns current time in JST (UTC+9)
        >>> date_now(0)  # Returns current time in UTC
        >>> date_now(1)  # Returns current time in UTC+1
    """
    utc_now = datetime.now(timezone.utc)
    target_tz = timezone(timedelta(hours=time_difference))
    return utc_now.astimezone(target_tz)
