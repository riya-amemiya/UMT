from collections.abc import Callable
from typing import TypeVar

T = TypeVar("T")
K = TypeVar("K")


def uniq_by(array: list[T], selector: Callable[[T], K]) -> list[T]:
    """
    Removes duplicate values from an array based on a selector function.

    Args:
        array: The array to process.
        selector: Function that returns the value to compare for uniqueness.

    Returns:
        A new array with unique values based on the selector.

    Example:
        >>> uniq_by([{'x': 1}, {'x': 2}, {'x': 1}], lambda item: item['x'])
        [{'x': 1}, {'x': 2}]
    """
    seen: set[K] = set()
    result: list[T] = []

    for item in array:
        key = selector(item)
        if key not in seen:
            seen.add(key)
            result.append(item)

    return result
