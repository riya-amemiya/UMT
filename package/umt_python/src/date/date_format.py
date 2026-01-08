import re
from datetime import datetime
from .get_timezone_offset_string import get_timezone_offset_string


def date_format(date: datetime, format_string: str = "YYYY-MM-DDTHH:mm:ssZ") -> str:
    """
    Converts a date to a string according to the specified format pattern.

    Args:
        date: The datetime object to format
        format_string: The format pattern string (default: "YYYY-MM-DDTHH:mm:ssZ")

    Returns:
        The formatted date string

    Available format tokens:
        YYYY: Full year (e.g., 2025)
        YY: Short year (e.g., 25)
        MM: Month with leading zero (01-12)
        M: Month without leading zero (1-12)
        DD: Day with leading zero (01-31)
        D: Day without leading zero (1-31)
        d: Day of week (0-6)
        HH: Hours with leading zero (00-23)
        H: Hours without leading zero (0-23)
        hh: Hours (12-hour) with leading zero (01-12)
        h: Hours (12-hour) without leading zero (1-12)
        mm: Minutes with leading zero (00-59)
        m: Minutes without leading zero (0-59)
        ss: Seconds with leading zero (00-59)
        s: Seconds without leading zero (0-59)
        SSS: Milliseconds with leading zeros (000-999)
        A: AM/PM
        a: am/pm
        Z: Timezone offset (+09:00)
        ZZ: Timezone offset without colon (+0900)

    Example:
        >>> from datetime import datetime
        >>> date_format(datetime(2025, 4, 4), 'YYYY-MM-DD')
        '2025-04-04'
    """
    if not isinstance(date, datetime):
        raise TypeError("Invalid Date in format")

    hours = date.hour
    year_string = str(date.year)
    month_string = str(date.month)
    date_string = str(date.day)
    hour_string = str(hours)
    minute_string = str(date.minute)
    second_string = str(date.second)
    millisecond_string = str(date.microsecond // 1000)
    day_string = str(date.weekday())
    ampm = "AM" if hours < 12 else "PM"
    timezone_offset_string = get_timezone_offset_string(date)

    matches = {
        "YY": year_string[-2:],
        "YYYY": year_string.zfill(4),
        "M": month_string,
        "MM": month_string.zfill(2),
        "D": date_string,
        "DD": date_string.zfill(2),
        "d": day_string,
        "H": hour_string,
        "HH": hour_string.zfill(2),
        "h": str(hours % 12 or 12),
        "hh": str(hours % 12 or 12).zfill(2),
        "a": ampm.lower(),
        "A": ampm,
        "m": minute_string,
        "mm": minute_string.zfill(2),
        "s": second_string,
        "ss": second_string.zfill(2),
        "SSS": millisecond_string.zfill(3),
        "Z": timezone_offset_string,
        "ZZ": timezone_offset_string.replace(":", ""),
    }

    def replacer(match: re.Match[str]) -> str:
        escaped = match.group(1)
        token = match.group(2)
        if escaped:
            return escaped
        return matches.get(token) or match.group(0)

    pattern = r"\[([^\]]+)]|(Y{1,4}|M{1,2}|D{1,2}|d{1,2}|H{1,2}|h{1,2}|a|A|m{1,2}|s{1,2}|Z{1,2}|SSS)"
    return re.sub(pattern, replacer, format_string)
