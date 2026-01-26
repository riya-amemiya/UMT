from typing import TypeVar

T = TypeVar("T")


def pop(array: list[T]) -> T | None:
    """
    Removes the last element from an array and returns it.

    Args:
        array: The array to remove the last element from.

    Returns:
        The last element of the array, or None if the array is empty.

    Example:
        >>> pop([1, 2, 3])
        3
        >>> pop([])
        >>> pop(['a', 'b'])
        'b'
    """
    if len(array) == 0:
        return None
    return list(array).pop()
