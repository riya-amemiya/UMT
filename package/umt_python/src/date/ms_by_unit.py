from typing import Literal

from ..consts.clock_consts import (
    ONE_DAY_MS,
    ONE_HOUR_MS,
    ONE_MINUTE_MS,
    ONE_SECOND_MS,
    ONE_WEEK_MS,
)

DurationUnit = Literal["ms", "s", "m", "h", "d", "w", "M", "y"]
"""Supported duration units for date arithmetic.

- ``ms``: milliseconds
- ``s``: seconds
- ``m``: minutes
- ``h``: hours
- ``d``: days
- ``w``: weeks
- ``M``: months (calendar-aware)
- ``y``: years (calendar-aware)
"""


ms_by_unit: dict[str, int] = {
    "ms": 1,
    "s": ONE_SECOND_MS,
    "m": ONE_MINUTE_MS,
    "h": ONE_HOUR_MS,
    "d": ONE_DAY_MS,
    "w": ONE_WEEK_MS,
}
"""Mapping of fixed-length duration units to their millisecond value.

Calendar-aware units (``M`` months and ``y`` years) are intentionally absent,
mirroring the TypeScript ``Partial<Record<DurationUnit, number>>``. A lookup of
those units returns ``None`` and signals the calendar-aware code path.

Example:
    >>> ms_by_unit["s"]
    1000
    >>> ms_by_unit.get("M") is None
    True
"""
