from collections.abc import Callable
from typing import TypeVar

T = TypeVar("T")


def take_while(
    array: list[T],
    predicate: Callable[[T, int, list[T]], bool],
) -> list[T]:
    """
    Returns the leading elements of an array while the predicate returns True.

    Args:
        array: The array to scan.
        predicate: Function evaluated for each element (value, index, array)
            until it first returns False.

    Returns:
        A new list of the leading elements that satisfy the predicate.

    Example:
        >>> take_while([1, 2, 3, 4, 1], lambda n, i, arr: n < 3)
        [1, 2]
    """
    for index, value in enumerate(array):
        if not predicate(value, index, array):
            return array[:index]
    return list(array)
