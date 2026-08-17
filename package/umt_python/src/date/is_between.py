from datetime import datetime

from .date_inclusivity import DateInclusivity
from .start_of import DateBoundaryUnit, start_of


def is_between(
    date: datetime,
    start: datetime,
    end: datetime,
    unit: DateBoundaryUnit | None = None,
    inclusivity: DateInclusivity = "()",
) -> bool:
    """
    Return True when `date` lies between `start` and `end`.

    Without a unit, compare exact timestamps. With a unit, truncate all
    three dates with `start_of` first. Ranges are not swapped.

    Args:
        date: Date to test
        start: Range start
        end: Range end
        unit: Optional boundary unit
        inclusivity: Bound inclusivity (default exclusive)

    Returns:
        True when the date is between the bounds

    Example:
        >>> from datetime import datetime
        >>> is_between(datetime(2025, 4, 15), datetime(2025, 4, 10), datetime(2025, 4, 20))
        True
    """
    value = date.timestamp() if unit is None else start_of(date, unit).timestamp()
    from_ts = start.timestamp() if unit is None else start_of(start, unit).timestamp()
    to_ts = end.timestamp() if unit is None else start_of(end, unit).timestamp()
    include_start = inclusivity.startswith("[")
    include_end = inclusivity.endswith("]")
    after_start = value >= from_ts if include_start else value > from_ts
    before_end = value <= to_ts if include_end else value < to_ts
    return after_start and before_end
