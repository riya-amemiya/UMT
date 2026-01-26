def mode(array: list[float]) -> list[float]:
    """
    Finds the most frequently occurring value(s) in an array.

    Args:
        array: Array of numbers to find mode for.

    Returns:
        Array of mode values (can be multiple values if there are ties).

    Example:
        >>> mode([1, 2, 2, 3, 3, 3])
        [3]
        >>> mode([1, 2, 2, 3, 3])
        [2, 3]
        >>> mode([1, 2, 3])
        [1, 2, 3]
    """
    if len(array) == 0:
        return []

    frequency: dict[float, int] = {}
    max_frequency = 0

    # Count frequencies
    for value in array:
        count = frequency.get(value, 0) + 1
        frequency[value] = count
        max_frequency = max(max_frequency, count)

    # Find all values with maximum frequency
    modes: list[float] = []
    for value, count in frequency.items():
        if count == max_frequency:
            modes.append(value)

    return sorted(modes)
