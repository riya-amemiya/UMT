import math
from datetime import datetime, timezone

from ..consts.clock_consts import (
    ONE_DAY_MS,
    ONE_HOUR_MS,
    ONE_MINUTE_MS,
    ONE_MONTH_MS,
    ONE_SECOND_MS,
    ONE_WEEK_MS,
    ONE_YEAR_MS,
)

_THRESHOLDS: list[tuple[str, int]] = [
    ("year", ONE_YEAR_MS),
    ("month", ONE_MONTH_MS),
    ("week", ONE_WEEK_MS),
    ("day", ONE_DAY_MS),
    ("hour", ONE_HOUR_MS),
    ("minute", ONE_MINUTE_MS),
    ("second", ONE_SECOND_MS),
]

# Replicates Intl.RelativeTimeFormat(..., { numeric: "auto" }) output for the
# "en" and "ja" locales (CLDR special words for small offsets plus the generic
# plural forms). Other locales fall back to an English-style generic form.
_SPECIAL_EN: dict[str, dict[int, str]] = {
    "second": {0: "now"},
    "minute": {0: "this minute"},
    "hour": {0: "this hour"},
    "day": {-1: "yesterday", 0: "today", 1: "tomorrow"},
    "week": {-1: "last week", 0: "this week", 1: "next week"},
    "month": {-1: "last month", 0: "this month", 1: "next month"},
    "year": {-1: "last year", 0: "this year", 1: "next year"},
}

_SPECIAL_JA: dict[str, dict[int, str]] = {
    "second": {0: "今"},
    "minute": {0: "1 分以内"},
    "hour": {0: "1 時間以内"},
    "day": {-2: "一昨日", -1: "昨日", 0: "今日", 1: "明日", 2: "明後日"},
    "week": {-1: "先週", 0: "今週", 1: "来週"},
    "month": {-1: "先月", 0: "今月", 1: "来月"},
    "year": {-1: "昨年", 0: "今年", 1: "来年"},
}

_JA_UNIT: dict[str, str] = {
    "second": "秒",
    "minute": "分",
    "hour": "時間",
    "day": "日",
    "week": "週間",
    "month": "か月",
    "year": "年",
}


def _round_half_up(value: float) -> int:
    return math.floor(value + 0.5)


def _format_en(value: int, unit: str) -> str:
    special = _SPECIAL_EN.get(unit, {})
    if value in special:
        return special[value]
    plural = unit if abs(value) == 1 else f"{unit}s"
    if value < 0:
        return f"{abs(value)} {plural} ago"
    return f"in {value} {plural}"


def _format_ja(value: int, unit: str) -> str:
    special = _SPECIAL_JA.get(unit, {})
    if value in special:
        return special[value]
    suffix = _JA_UNIT[unit]
    if value < 0:
        return f"{abs(value)} {suffix}前"
    return f"{value} {suffix}後"


def format_relative(
    date: datetime,
    base_date: datetime | None = None,
    locale: str | None = None,
) -> str:
    """
    Format a date relative to a base date, mirroring Intl.RelativeTimeFormat
    with ``numeric: "auto"``.

    Picks the largest unit that fits the absolute delta. Full CLDR fidelity is
    provided for the "en" and "ja" locales; other locales fall back to an
    English-style generic form.

    Args:
        date: The target date
        base_date: Reference date (defaults to now)
        locale: BCP 47 locale tag; uses English when omitted or unsupported

    Returns:
        Localized relative-time string

    Example:
        >>> from datetime import datetime
        >>> base = datetime(2025, 4, 15, 12, 0, 0)
        >>> from datetime import timedelta
        >>> format_relative(base - timedelta(hours=1), base, "en")
        '1 hour ago'
        >>> format_relative(base + timedelta(days=1), base, "en")
        'tomorrow'
    """
    if base_date is None:
        base_date = datetime.now(timezone.utc).astimezone().replace(tzinfo=None)

    delta = date - base_date
    delta_ms = (
        delta.days * 86_400_000 + delta.seconds * 1000 + delta.microseconds // 1000
    )
    abs_delta = abs(delta_ms)

    formatter = _format_ja if locale == "ja" else _format_en

    for unit, ms in _THRESHOLDS:
        if abs_delta >= ms:
            value = _round_half_up(delta_ms / ms)
            return formatter(value, unit)
    return formatter(0, "second")
