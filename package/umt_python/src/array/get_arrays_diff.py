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
    all_values: set[Any] = set(array)
    duplicates: set[Any] = set()

    for arr in arrays:
        for value in arr:
            if value in all_values:
                duplicates.add(value)
            else:
                all_values.add(value)

    return [value for value in all_values if value not in duplicates]
