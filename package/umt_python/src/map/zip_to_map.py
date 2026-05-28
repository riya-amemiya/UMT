from collections.abc import Hashable
from typing import TypeVar

K = TypeVar("K", bound=Hashable)
V = TypeVar("V")


def zip_to_map(keys: list[K], values: list[V]) -> dict[K, V]:
    """
    Creates a dict from two arrays, using the first as keys and the second as values.

    Pairs are formed at corresponding positions; iteration stops at the shorter
    length, matching the behavior of zip. Duplicate keys keep the latest value.
    Python's dict preserves insertion order (3.7+), mirroring the TS Map semantics.

    Args:
        keys: List of keys.
        values: List of values.

    Returns:
        Dict pairing keys with values at the same index.

    Example:
        >>> zip_to_map(["a", "b"], [1, 2])
        {'a': 1, 'b': 2}
        >>> zip_to_map(["a", "b", "c"], [1, 2])
        {'a': 1, 'b': 2}
    """
    length = min(len(keys), len(values))
    result: dict[K, V] = {}
    for index in range(length):
        result[keys[index]] = values[index]
    return result
