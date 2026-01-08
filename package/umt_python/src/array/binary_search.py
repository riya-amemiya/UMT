def binary_search(array: list[int | float], target: int | float) -> int:
    """
    Binary search implementation.

    Args:
        array: A sorted array of numbers.
        target: The value to search for.

    Returns:
        The index of the target value in the array, or -1 if not found.

    Example:
        >>> binary_search([1, 2, 3, 4, 5], 3)
        2
        >>> binary_search([1, 2, 3, 4, 5], 6)
        -1
    """
    left = 0
    right = len(array) - 1

    while left <= right:
        mid = (left + right) // 2
        mid_value = array[mid]

        if mid_value == target:
            return mid
        if mid_value < target:
            left = mid + 1
        else:
            right = mid - 1

    return -1
