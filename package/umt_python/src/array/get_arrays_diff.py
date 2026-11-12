from typing import Any


def get_arrays_diff(array: list[Any], *arrays: list[Any]) -> list[Any]:
    """
    Extract elements that are not common between arrays.

    Args:
        array: The first array.
        *arrays: Additional arrays to compare.

    Returns:
        Array containing elements that appear only once across all arrays.

    Example:
        >>> get_arrays_diff([1, 2, 3], [2, 3, 4])
        [1, 4]
    """
    all_values: dict[Any, None] = dict.fromkeys(array)
    duplicates: set[Any] = set()

    for arr in arrays:
        for value in arr:
            if value in duplicates:
                continue
            if value in all_values:
                del all_values[value]
                duplicates.add(value)
            else:
                all_values[value] = None

    return list(all_values)
