import math
from typing import Any


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
    lookup_structures = []
    for arr in arrays:
        try:
            lookup_structures.append(set(arr))
        except TypeError:
            lookup_structures.append(arr)

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
        else:
            is_common = True
            for container in lookup_structures:
                if isinstance(container, set):
                    try:
                        if item not in container:
                            is_common = False
                            break
                    except TypeError:
                        # item is unhashable, so it cannot be in the set
                        # (which only contains hashable items)
                        is_common = False
                        break
                elif item not in container:
                    is_common = False
                    break

            if is_common:
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
            try:
                if item not in seen:
                    seen.add(item)
                    unique_result.append(item)
            except TypeError:
                if item not in unique_result:
                    unique_result.append(item)

    return unique_result
