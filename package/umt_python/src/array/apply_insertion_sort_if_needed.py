from typing import Callable, TypeVar

from .insertion_sort_range import insertion_sort_range

T = TypeVar("T")

CompareFunction = Callable[[T, T], int | float]


def apply_insertion_sort_if_needed(
    array: list[T],
    low: int,
    high: int,
    compare_function: CompareFunction[T],
    insertion_sort_threshold: int,
) -> bool:
    """
    Checks if a partition is small enough to apply insertion sort and applies it if so.

    Args:
        array: The array containing the partition.
        low: The starting index of the partition.
        high: The ending index of the partition.
        compare_function: The function to compare elements.
        insertion_sort_threshold: The size threshold for switching to insertion sort.

    Returns:
        True if insertion sort was applied, False otherwise.

    Example:
        >>> arr = [5, 2, 4, 1, 3]
        >>> apply_insertion_sort_if_needed(arr, 0, 4, lambda a, b: a - b, 10)
        True
        >>> arr
        [1, 2, 3, 4, 5]
        >>> arr2 = [5, 2, 4, 1, 3]
        >>> apply_insertion_sort_if_needed(arr2, 0, 4, lambda a, b: a - b, 3)
        False
        >>> arr2
        [5, 2, 4, 1, 3]
    """
    if high - low + 1 <= insertion_sort_threshold:
        insertion_sort_range(array, compare_function, low, high)
        return True
    return False
