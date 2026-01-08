from typing import TypeVar

T = TypeVar("T")


def first(array: list[T]) -> T | None:
    """
    Returns the first element of an array.

    Args:
        array: The input array.

    Returns:
        The first element of the array, or None if the array is empty.

    Example:
        >>> first([1, 2, 3])
        1
        >>> first([])
        >>> first(['a', 'b', 'c'])
        'a'
    """
    return array[0] if len(array) > 0 else None
