from collections.abc import Callable
from typing import TypeVar

T = TypeVar("T")
K = TypeVar("K", str, int)


def count_by(
    array: list[T],
    iteratee: Callable[[T, int, list[T]], K],
) -> dict[K, int]:
    """
    Counts elements of an array grouped by an iteratee key.

    Args:
        array: Array to count.
        iteratee: Function to determine the group key for each element.

    Returns:
        Object mapping keys to counts.

    Example:
        >>> count_by([6.1, 4.2, 6.3], lambda x, i, arr: int(x))
        {6: 2, 4: 1}
        >>> count_by(["one", "two", "three"], lambda x, i, arr: len(x))
        {3: 2, 5: 1}
    """
    result: dict[K, int] = {}
    for index, value in enumerate(array):
        key = iteratee(value, index, array)
        result[key] = result.get(key, 0) + 1
    return result
