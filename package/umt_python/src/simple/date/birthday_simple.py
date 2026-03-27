from datetime import datetime

from src.date.birthday import birthday


def _parse_date_string(date_str: str) -> tuple[int, int, int] | None:
    """Parse a date string with -, :, or / delimiters."""
    for delimiter in (":", "/", "-"):
        if delimiter in date_str:
            parts = date_str.split(delimiter)
            if len(parts) == 3:
                try:
                    return int(parts[0]), int(parts[1]), int(parts[2])
                except ValueError:
                    return None
    return None


def birthday_simple(
    birthdays: str | dict[str, int] | datetime,
    time_difference: int = 9,
) -> int:
    """
    Calculate age from birthdate with flexible input formats.

    Accepts multiple input formats for convenience, matching the
    flexible interface of the TypeScript/Rust versions.

    Args:
        birthdays: Birthday in one of these formats:
            - String: "YYYY-MM-DD", "YYYY:MM:DD", or "YYYY/MM/DD"
            - Dict: {"year": int, "mon": int, "day": int}
            - datetime object
        time_difference: Hours offset from UTC (default: 9 for JST)

    Returns:
        Age in years (0 if birthday is in the future or parsing fails).

    Example:
        >>> birthday_simple("2000-01-01")
        >>> birthday_simple("2000:01:01")
        >>> birthday_simple({"year": 2000, "mon": 1, "day": 1})
    """
    if isinstance(birthdays, str):
        parsed = _parse_date_string(birthdays)
        if parsed is not None:
            year, mon, day = parsed
            return birthday(year, mon, day, time_difference)
        return 0

    if isinstance(birthdays, datetime):
        return birthday(
            birthdays.year,
            birthdays.month,
            birthdays.day,
            time_difference,
        )

    if isinstance(birthdays, dict):
        return birthday(
            birthdays["year"],
            birthdays["mon"],
            birthdays["day"],
            time_difference,
        )

    return 0
