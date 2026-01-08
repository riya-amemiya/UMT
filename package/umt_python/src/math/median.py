def median(array: list[float]) -> float:
    """
    Calculate the median of an array of numbers.

    Args:
        array: Array of numbers.

    Returns:
        The median value.

    Example:
        >>> median([1, 3, 3, 6, 7, 8, 9])
        6
        >>> median([1, 2, 3, 4])
        2.5
    """
    sorted_array = sorted(array)
    mid = len(sorted_array) // 2

    if len(sorted_array) % 2 == 0:
        return (sorted_array[mid - 1] + sorted_array[mid]) / 2
    return sorted_array[mid]
