def array_sum(x: list[int | float]) -> int | float:
    """
    Returns the sum of an array of numbers.

    Args:
        x: Array of numbers.

    Returns:
        Sum of the array elements.

    Example:
        >>> array_sum([1, 2, 3])
        6
        >>> array_sum([1.5, 2.5, 3.0])
        7.0
    """
    result: int | float = 0
    for num in x:
        result += num
    return result
