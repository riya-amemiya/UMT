from datetime import datetime, timedelta


def date_range(start_date: datetime, end_date: datetime) -> list[datetime]:
    """
    Generate a list containing all dates between the specified start and end dates.

    Args:
        start_date: The start date of the range
        end_date: The end date of the range

    Returns:
        A list of datetime objects from start_date to end_date (inclusive)

    Example:
        >>> from datetime import datetime
        >>> date_range(datetime(2025, 1, 1), datetime(2025, 1, 3))
        [datetime(2025, 1, 1), datetime(2025, 1, 2), datetime(2025, 1, 3)]
    """
    dates: list[datetime] = []
    current_date = start_date
    while current_date <= end_date:
        dates.append(current_date)
        current_date = current_date + timedelta(days=1)
    return dates
