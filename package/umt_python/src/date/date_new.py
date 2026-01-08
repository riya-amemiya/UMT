from datetime import datetime, timezone


def new_date_int(
    year: int,
    mon: int,
    day: int,
    hours: int | None = None,
    minutes: int = 0,
    seconds: int = 0,
    milliseconds: int = 0,
) -> datetime:
    """
    Create a new datetime object from numeric values.

    Args:
        year: The year
        mon: The month (1-12)
        day: The day of the month
        hours: Hours offset from UTC (defaults to local timezone offset)
        minutes: Minutes (0-59)
        seconds: Seconds (0-59)
        milliseconds: Milliseconds (0-999)

    Returns:
        datetime object

    Example:
        >>> new_date_int(2021, 1, 1)  # Creates date for January 1, 2021
    """
    if hours is None:
        local_now = datetime.now()
        utc_now = datetime.now(timezone.utc).replace(tzinfo=None)
        hours = round((local_now - utc_now).total_seconds() / 3600)

    return datetime(
        year=year,
        month=mon,
        day=day,
        hour=hours,
        minute=minutes,
        second=seconds,
        microsecond=milliseconds * 1000,
    )


def new_date_string(
    date_str: str,
    hours: str = "00",
    minutes: str = "00",
    seconds: str = "00",
    milliseconds: str = "000",
    time_difference: str = "00",
) -> datetime:
    """
    Create a new datetime object from a string date and time components.

    Args:
        date_str: Date string in format "YYYY-MM-DD"
        hours: Hours in "HH" format (00-23)
        minutes: Minutes in "mm" format (00-59)
        seconds: Seconds in "ss" format (00-59)
        milliseconds: Milliseconds in "mmm" format (000-999)
        time_difference: Timezone offset in "HH" format (e.g., "09" for UTC+9)

    Returns:
        datetime object

    Example:
        >>> new_date_string("2021-01-01")  # Creates date for January 1, 2021 00:00:00
    """
    iso_string = (
        f"{date_str}T{hours}:{minutes}:{seconds}.{milliseconds}+{time_difference}:00"
    )
    return datetime.fromisoformat(iso_string)
