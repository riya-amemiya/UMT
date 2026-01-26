from collections.abc import Callable
from typing import TypeVar

T = TypeVar("T")

CompareFunction = Callable[[T, T], int | float]


def _compare_function_default(a: T, b: T) -> int | float:
    """Default comparison function."""
    if a < b:  # type: ignore
        return -1
    if a > b:  # type: ignore
        return 1
    return 0


def _insertion_sort_range(
    array: list[T],
    start: int,
    end: int,
    compare_function: CompareFunction[T],
) -> None:
    """Sort a range using insertion sort."""
    for i in range(start + 1, end + 1):
        key = array[i]
        j = i - 1
        while j >= start and compare_function(array[j], key) > 0:
            array[j + 1] = array[j]
            j -= 1
        array[j + 1] = key


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
        compare_function = _compare_function_default

    if end is None:
        end = len(result) - 1

    _insertion_sort_range(result, start, end, compare_function)
    return result
