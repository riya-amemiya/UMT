from typing import Literal, Union, overload

TimeUnit = Literal["milliseconds", "seconds", "minutes", "hours"]
TimeUnitShort = Literal["ms", "s", "m", "h"]

TIME_UNIT_MAP = {
    "milliseconds": {"long": "milliseconds", "short": "ms"},
    "seconds": {"long": "seconds", "short": "s"},
    "minutes": {"long": "minutes", "short": "m"},
    "hours": {"long": "hours", "short": "h"},
    "ms": {"long": "milliseconds", "short": "ms"},
    "s": {"long": "seconds", "short": "s"},
    "m": {"long": "minutes", "short": "m"},
    "h": {"long": "hours", "short": "h"},
}


@overload
def normalize_time_unit(
    unit: Union[TimeUnit, TimeUnitShort], to: Literal["long"]
) -> TimeUnit: ...


@overload
def normalize_time_unit(
    unit: Union[TimeUnit, TimeUnitShort], to: Literal["short"]
) -> TimeUnitShort: ...


def normalize_time_unit(
    unit: Union[TimeUnit, TimeUnitShort], to: Literal["long", "short"]
) -> Union[TimeUnit, TimeUnitShort]:
    """
    Normalize time unit.

    Args:
        unit: Time unit
        to: "long" or "short"

    Returns:
        Normalized time unit

    Example:
        >>> normalize_time_unit("ms", "long")
        'milliseconds'
        >>> normalize_time_unit("hours", "short")
        'h'
    """
    return TIME_UNIT_MAP[unit][to]  # type: ignore
