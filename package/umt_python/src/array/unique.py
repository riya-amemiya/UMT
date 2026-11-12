from typing import TypeVar

from .ordered_unique import ordered_unique

T = TypeVar("T")


def unique(array: list[T]) -> list[T]:
    """
    Removes duplicate values from an array.

    Args:
        array: The array to process.

    Returns:
        A new array with unique values.

    Example:
        >>> unique([1, 2, 2, 3, 3, 3])
        [1, 2, 3]
        >>> unique(['a', 'b', 'a', 'c'])
        ['a', 'b', 'c']
    """
    return ordered_unique(array)
