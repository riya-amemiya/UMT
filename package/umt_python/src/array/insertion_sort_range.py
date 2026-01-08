from typing import Callable, TypeVar

T = TypeVar("T")

CompareFunction = Callable[[T, T], int | float]


def insertion_sort_range(
    array: list[T],
    compare_function: CompareFunction[T],
    start: int,
    end: int,
) -> None:
    """
    Sorts a range of an array in-place using insertion sort.

    Args:
        array: The array containing the range to sort.
        compare_function: Function to compare elements.
        start: The starting index of the range.
        end: The ending index of the range (inclusive).

    Returns:
        None. The array is modified in-place.

    Example:
        >>> arr = [5, 2, 4, 1, 3]
        >>> insertion_sort_range(arr, lambda a, b: a - b, 0, 4)
        >>> arr
        [1, 2, 3, 4, 5]
        >>> arr2 = [3, 1, 4, 1, 5, 9, 2, 6]
        >>> insertion_sort_range(arr2, lambda a, b: a - b, 2, 5)
        >>> arr2
        [3, 1, 1, 4, 5, 9, 2, 6]
    """
    for index in range(start + 1, end + 1):
        target = array[index]
        j = index
        while j > start and compare_function(array[j - 1], target) > 0:
            array[j] = array[j - 1]
            j -= 1
        array[j] = target
