from collections.abc import Callable
from typing import TypeVar

T = TypeVar("T")
R = TypeVar("R")


def object_map_values(
    obj: dict[str, T],
    func: Callable[[T, str], R],
) -> dict[str, R]:
    """
    Creates an object with the same keys as the given object, but
    with values transformed by the provided function.

    Args:
        obj: The source object to map values from.
        func: The function invoked per value. Receives (value, key).

    Returns:
        A new object with transformed values.

    Example:
        >>> object_map_values({"a": 1, "b": 2}, lambda v, k: v * 2)
        {'a': 2, 'b': 4}
    """
    result: dict[str, R] = {}
    for key, value in obj.items():
        result[key] = func(value, key)
    return result
