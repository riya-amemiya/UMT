from datetime import datetime

from .add_duration import add_duration


def sub_duration(date: datetime, amount: int | float, unit: str) -> datetime:
    """
    Subtract a duration from a date and return a new datetime.

    Equivalent to ``add_duration(date, -amount, unit)``.

    Args:
        date: Base date
        amount: Amount to subtract
        unit: Unit of the amount
            ("ms"|"s"|"m"|"h"|"d"|"w"|"M"|"y")

    Returns:
        A new datetime with the duration subtracted

    Example:
        >>> from datetime import datetime
        >>> sub_duration(datetime(2025, 3, 31), 1, "M")
        datetime.datetime(2025, 2, 28, 0, 0)
    """
    return add_duration(date, -amount, unit)
