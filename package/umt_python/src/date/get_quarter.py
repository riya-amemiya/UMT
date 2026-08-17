from datetime import datetime


def get_quarter(date: datetime) -> int:
    """
    Return the calendar quarter (1-4) for the date's month.

    Matches start_of(..., "quarter"): Jan-Mar=1, Apr-Jun=2, Jul-Sep=3,
    Oct-Dec=4.

    Args:
        date: Date to inspect

    Returns:
        Quarter number from 1 to 4

    Example:
        >>> from datetime import datetime
        >>> get_quarter(datetime(2025, 4, 15))
        2
    """
    return (date.month - 1) // 3 + 1
