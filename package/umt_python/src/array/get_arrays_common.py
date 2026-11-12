import math
from typing import Any

from .ordered_unique import ordered_unique


def _to_set_if_hashable(arr: list[Any]) -> set[Any] | list[Any]:
    try:
        return set(arr)
    except TypeError:
        return arr


def _safe_is_in(item: object, container: set[Any] | list[Any]) -> bool:
    if isinstance(container, set):
        try:
            return item in container
        except TypeError:
            return False
    return item in container


class _EqualityKey:
    __slots__ = ("value",)

    def __init__(self, value: object) -> None:
        self.value = value

    def __hash__(self) -> int:
        return 0

    def __eq__(self, other: object) -> bool:
        return isinstance(other, _EqualityKey) and self.value == other.value


_NAN_KEY = object()


def _dedupe_key(item: object) -> object:
    if isinstance(item, float) and math.isnan(item):
        return _NAN_KEY
    try:
        hash(item)
    except TypeError:
        return _EqualityKey(item)
    return item


def get_arrays_common(array: list[Any], *arrays: list[Any]) -> list[Any]:
    """
    Extract common elements from multiple arrays.

    Args:
        array: The first array.
        *arrays: Additional arrays to compare.

    Returns:
        Array containing common elements.

    Example:
        >>> get_arrays_common([1, 2, 3], [2, 3, 4], [2, 5, 3])
        [2, 3]
    """
    if len(arrays) == 0:
        return list(array)

    # Pre-process arrays for O(1) lookups where possible
    lookup_structures = [_to_set_if_hashable(arr) for arr in arrays]

    result = []
    for item in array:
        if isinstance(item, float) and math.isnan(item):
            if all(
                any(
                    isinstance(arr_item, float) and math.isnan(arr_item)
                    for arr_item in arr
                )
                for arr in arrays
            ):
                result.append(item)
        elif all(_safe_is_in(item, container) for container in lookup_structures):
            result.append(item)

    return ordered_unique(result, _dedupe_key)
