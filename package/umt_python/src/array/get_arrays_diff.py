import itertools
from typing import Any

from .ordered_unique import ordered_unique


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
    duplicates: set[Any] = set()
    merged = ordered_unique(
        itertools.chain(ordered_unique(array), *arrays),
        on_duplicate=duplicates.add,
    )
    return [value for value in merged if value not in duplicates]
