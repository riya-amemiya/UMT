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
        elif all(item in arr for arr in arrays):
            result.append(item)

    unique_result: list[Any] = []
    for item in result:
        if isinstance(item, float) and math.isnan(item):
            if not any(
                isinstance(existing, float) and math.isnan(existing)
                for existing in unique_result
            ):
                unique_result.append(item)
        elif item not in unique_result:
            unique_result.append(item)

    return unique_result
