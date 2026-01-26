import itertools
from typing import Any


def arrays_join(array: list[Any], *arrays: list[Any]) -> list[Any]:
    """
    Join arrays without duplicates.

    Args:
        array: First array to join.
        *arrays: Additional arrays to join.

    Returns:
        Array with unique elements from all input arrays.

    Example:
        >>> arrays_join([1, 2, 3], [2, 3, 4])
        [1, 2, 3, 4]
        >>> arrays_join([1, 2], [2, 3], [3, 4])
        [1, 2, 3, 4]
    """
    seen: set[Any] = set()
    result: list[Any] = []
    for item in itertools.chain(array, *arrays):
        if item not in seen:
            seen.add(item)
            result.append(item)
    return result
