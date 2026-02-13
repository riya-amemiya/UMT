def in_range(value: float, start: float, end: float | None = None) -> bool:
    """
    Checks if a number is within a given range.
    If only two arguments are provided, the range is [0, start).
    If three arguments are provided, the range is [min(start, end), max(start, end)).

    Args:
        value (float): The number to check.
        start (float): The start of the range, or the end if end is not provided.
        end (float, optional): The end of the range.

    Returns:
        bool: True if the value is within the range, False otherwise.

    Example:
        >>> in_range(3, 5)
        True
        >>> in_range(5, 5)
        False
        >>> in_range(3, 2, 5)
        True
        >>> in_range(3, 5, 2)
        True
    """
    if end is None:
        lower = min(start, 0)
        upper = max(start, 0)
    else:
        lower = min(start, end)
        upper = max(start, end)

    return lower <= value < upper
