import math
from typing import Any


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


def _add_unique(item: object, seen: set[Any], unique_result: list[Any]) -> None:
    try:
        if item not in seen:
            seen.add(item)
            unique_result.append(item)
    except TypeError:
        if item not in unique_result:
            unique_result.append(item)


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

    unique_result: list[Any] = []
    seen: set[Any] = set()
    has_seen_nan = False

    for item in result:
        if isinstance(item, float) and math.isnan(item):
            if not has_seen_nan:
                unique_result.append(item)
                has_seen_nan = True
        else:
            _add_unique(item, seen, unique_result)

    return unique_result
