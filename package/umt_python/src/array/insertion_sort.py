from collections.abc import Callable
from typing import TypeVar

from .compare_function_default import compare_function_default
from .insertion_sort_range import insertion_sort_range

T = TypeVar("T")

CompareFunction = Callable[[T, T], int | float]


def insertion_sort(
    array: list[T],
    compare_function: CompareFunction[T] | None = None,
    start: int = 0,
    end: int | None = None,
) -> list[T]:
    """
    Sort an array using insertion sort algorithm.

    Args:
        array: Array to sort.
        compare_function: Function to compare two elements.
        start: Starting index for sorting (inclusive).
        end: Ending index for sorting (inclusive).

    Returns:
        Sorted array.

    Example:
        >>> insertion_sort([4, 2, 7, 1, 3])
        [1, 2, 3, 4, 7]
        >>> insertion_sort([4, 2, 7, 1, 3], lambda a, b: a - b)
        [1, 2, 3, 4, 7]
    """
    result = list(array)
    if len(result) <= 1:
        return result

    if compare_function is None:
        compare_function = compare_function_default

    if end is None:
        end = len(result) - 1

    insertion_sort_range(result, compare_function, start, end)
    return result
