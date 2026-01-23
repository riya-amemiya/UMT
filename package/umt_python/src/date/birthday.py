from datetime import datetime, timedelta, timezone


def birthday(year: int, mon: int, day: int, time_difference: int = 9) -> int:
    """
    Calculate age based on birthdate.

    Args:
        year: Birth year
        mon: Birth month (1-12)
        day: Birth day
        time_difference: Time difference from UTC in hours (default: 9)

    Returns:
        Age in years

    Example:
        >>> birthday(2000, 1, 1)  # Returns age of someone born on Jan 1, 2000
    """
    target_tz = timezone(timedelta(hours=time_difference))
    birthday_date = datetime(year, mon, day, tzinfo=target_tz)
    now_time = datetime.now(target_tz)

    current_year = now_time.year
    birth_year = birthday_date.year

    age = current_year - birth_year

    this_year_birthday = datetime(
        current_year, birthday_date.month, birthday_date.day, tzinfo=target_tz
    )
    if now_time < this_year_birthday:
        age -= 1

    if age < 0:
        return 0

    return age
