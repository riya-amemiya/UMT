from datetime import datetime

from src.date.day_of_week import day_of_week


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


def day_of_week_simple(
    properties: str | dict[str, int | None] | datetime | None = None,
    time_difference: int = 9,
) -> int:
    """
    Get the day of the week for a given date.

    Accepts multiple input formats for convenience, matching the
    flexible interface of the TypeScript/Rust versions.

    Args:
        properties: Date specification in one of these formats:
            - String: "YYYY-MM-DD", "YYYY:MM:DD", or "YYYY/MM/DD"
            - Dict: {"year": int, "mon": int, "day": int} (all optional)
            - datetime object
            - None (defaults to current date)
        time_difference: Hours offset from UTC (default: 9 for JST)

    Returns:
        Day of the week as int (0 = Monday, 6 = Sunday per Python convention).
        Note: Python uses 0=Monday unlike JS/Rust which use 0=Sunday.

    Example:
        >>> day_of_week_simple("2022-01-03")  # Monday -> 0
        0
        >>> day_of_week_simple("2022/01/02")  # Sunday -> 6
        6
        >>> day_of_week_simple({"year": 2022, "mon": 1, "day": 5})  # Wednesday -> 2
        2
    """
    if properties is None:
        return day_of_week(time_difference=time_difference)

    if isinstance(properties, str):
        parsed = _parse_date_string(properties)
        if parsed is not None:
            year, mon, day = parsed
            return day_of_week(year, mon, day, time_difference)
        return day_of_week(time_difference=time_difference)

    if isinstance(properties, datetime):
        return day_of_week(
            properties.year,
            properties.month,
            properties.day,
            time_difference,
        )

    if isinstance(properties, dict):
        return day_of_week(
            properties.get("year"),
            properties.get("mon"),
            properties.get("day"),
            time_difference,
        )

    return day_of_week(time_difference=time_difference)
