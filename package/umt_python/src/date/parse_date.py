import calendar
import re
from dataclasses import dataclass
from datetime import datetime, timedelta, timezone

_FORMAT_TOKEN_PATTERN = re.compile(
    r"\[([^\]]+)]|YYYY|YY|MM|M|DD|D|d|HH|H|hh|h|mm|m|ss|s|SSS|A|a|ZZ|Z|(.)",
    re.DOTALL,
)

_TOKEN_PATTERNS = {
    "YYYY": "[0-9]{4}",
    "YY": "[0-9]{2}",
    "MM": "[0-9]{2}",
    "M": "[0-9]{1,2}",
    "DD": "[0-9]{2}",
    "D": "[0-9]{1,2}",
    "d": "[0-6]",
    "HH": "[0-9]{2}",
    "H": "[0-9]{1,2}",
    "hh": "[0-9]{2}",
    "h": "[0-9]{1,2}",
    "mm": "[0-9]{2}",
    "m": "[0-9]{1,2}",
    "ss": "[0-9]{2}",
    "s": "[0-9]{1,2}",
    "SSS": "[0-9]{3}",
    "A": "AM|PM",
    "a": "am|pm",
    "ZZ": "[+-][0-9]{4}",
    "Z": "[+-][0-9]{2}:[0-9]{2}",
}


@dataclass
class _ParsedFields:
    year: int = 1970
    month: int = 1
    day: int = 1
    hour: int = 0
    minute: int = 0
    second: int = 0
    millisecond: int = 0
    twelve_hour: bool = False
    pm: bool | None = None
    day_of_week: int | None = None
    offset: int | None = None
    offset_in_range: bool = True


def _apply_token(fields: _ParsedFields, token: str, value: str) -> None:
    if token == "YYYY":
        fields.year = int(value)
    elif token == "YY":
        fields.year = 2000 + int(value)
    elif token in ("MM", "M"):
        fields.month = int(value)
    elif token in ("DD", "D"):
        fields.day = int(value)
    elif token == "d":
        fields.day_of_week = int(value)
    elif token in ("HH", "H"):
        fields.hour = int(value)
        fields.twelve_hour = False
    elif token in ("hh", "h"):
        fields.hour = int(value)
        fields.twelve_hour = True
    elif token in ("mm", "m"):
        fields.minute = int(value)
    elif token in ("ss", "s"):
        fields.second = int(value)
    elif token == "SSS":
        fields.millisecond = int(value)
    elif token in ("A", "a"):
        fields.pm = value.lower() == "pm"
    else:
        digits = value.replace(":", "")
        offset_hours = int(digits[1:3])
        offset_minutes = int(digits[3:5])
        fields.offset = (-1 if digits.startswith("-") else 1) * (
            offset_hours * 60 + offset_minutes
        )
        fields.offset_in_range = offset_hours <= 23 and offset_minutes <= 59


def parse_date(string: str, format_string: str) -> datetime | None:
    """
    Parse a string into a datetime according to a format pattern.

    Uses the same tokens as `date_format`: YYYY, YY, MM, M, DD, D, d, HH, H,
    hh, h, mm, m, ss, s, SSS, A, a, Z, ZZ, and `[text]` for escaped literal
    text. Any other character in the pattern must match literally.

    - `YY` is read as 2000-2099.
    - Fields missing from the pattern default to 1970-01-01 00:00:00.000.
    - `hh` / `h` are converted with `A` / `a` when present (12 AM is 0,
      12 PM is 12); without a meridiem token the hour is used as written.
    - `d` (day of week, 0 = Sunday) must agree with the parsed calendar date.
    - With `Z` / `ZZ`, the result is an aware datetime in that offset;
      otherwise it is a naive local datetime.
    - When a token appears more than once, the last occurrence wins.

    Returns None when the string does not match the pattern or a field is out
    of range (month 1-12, day within the month, hour 0-23 or 1-12 for 12-hour
    tokens, minute and second 0-59, offset hours 0-23 and minutes 0-59).
    Year 0 is also None because `datetime` cannot represent it.

    Args:
        string: The date string to parse
        format_string: The format pattern string

    Returns:
        The parsed datetime, or None when parsing fails

    Example:
        >>> parse_date("2025-04-04", "YYYY-MM-DD")
        datetime.datetime(2025, 4, 4, 0, 0)
        >>> parse_date("04/04/2025 3:30 PM", "MM/DD/YYYY h:mm A")
        datetime.datetime(2025, 4, 4, 15, 30)
        >>> parse_date("2025-04-04T15:30:00+09:00", "YYYY-MM-DDTHH:mm:ssZ")
        datetime.datetime(2025, 4, 4, 15, 30, tzinfo=datetime.timezone(datetime.timedelta(seconds=32400)))
        >>> parse_date("2025-13-01", "YYYY-MM-DD") is None
        True
    """
    tokens: list[str] = []

    def replacer(match: re.Match[str]) -> str:
        text = match.group(1) or match.group(2)
        if text is not None:
            return re.escape(text)
        tokens.append(match.group(0))
        return f"({_TOKEN_PATTERNS[match.group(0)]})"

    source = _FORMAT_TOKEN_PATTERN.sub(replacer, format_string)
    matched = re.fullmatch(source, string)
    if matched is None:
        return None

    fields = _ParsedFields()
    for token, value in zip(tokens, matched.groups(), strict=True):
        _apply_token(fields, token, value)

    is_valid = (
        fields.year >= 1
        and 1 <= fields.month <= 12
        and 1 <= fields.day <= calendar.monthrange(fields.year, fields.month)[1]
        and ((1 <= fields.hour <= 12) if fields.twelve_hour else fields.hour <= 23)
        and fields.minute <= 59
        and fields.second <= 59
        and (
            fields.day_of_week is None
            or fields.day_of_week
            == (calendar.weekday(fields.year, fields.month, fields.day) + 1) % 7
        )
        and fields.offset_in_range
    )
    if not is_valid:
        return None

    hour = (
        fields.hour % 12 + (12 if fields.pm else 0)
        if fields.twelve_hour and fields.pm is not None
        else fields.hour
    )
    tzinfo = (
        None if fields.offset is None else timezone(timedelta(minutes=fields.offset))
    )
    return datetime(
        fields.year,
        fields.month,
        fields.day,
        hour,
        fields.minute,
        fields.second,
        fields.millisecond * 1000,
        tzinfo=tzinfo,
    )
