from collections.abc import Callable, Hashable
from typing import TypeVar

T = TypeVar("T")
K = TypeVar("K", bound=Hashable)


def group_by_to_map(
    array: list[T],
    iteratee: Callable[[T, int, list[T]], K],
) -> dict[K, list[T]]:
    """
    Groups elements of a list into a dict based on a given iteratee function.

    Unlike group_by which returns a dict[str | int, list[T]], this preserves
    insertion order and allows any hashable key type. Python's dict preserves
    insertion order (3.7+), mirroring the TS Map semantics.

    Args:
        array: List to group.
        iteratee: Function to determine the group key for each element.

    Returns:
        Dict with grouped elements keyed by the iteratee result.

    Example:
        >>> group_by_to_map([6.1, 4.2, 6.3], lambda x, i, arr: int(x))
        {6: [6.1, 6.3], 4: [4.2]}
        >>> group_by_to_map(["one", "two", "three"], lambda x, i, arr: len(x))
        {3: ['one', 'two'], 5: ['three']}
    """
    result: dict[K, list[T]] = {}
    for index, value in enumerate(array):
        key = iteratee(value, index, array)
        bucket = result.get(key)
        if bucket is None:
            result[key] = [value]
        else:
            bucket.append(value)
    return result
