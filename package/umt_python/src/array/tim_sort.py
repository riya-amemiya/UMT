from typing import Callable, TypeVar

T = TypeVar("T")

CompareFunction = Callable[[T, T], int | float]

MIN_RUN = 32


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


def _merge(
    array: list[T],
    start: int,
    mid: int,
    end: int,
    compare_function: CompareFunction[T],
) -> None:
    """Merges two sorted portions of the array."""
    left = array[start : mid + 1]
    right = array[mid + 1 : end + 1]
    left_index = 0
    right_index = 0
    array_index = start

    while left_index < len(left) and right_index < len(right):
        if compare_function(left[left_index], right[right_index]) <= 0:
            array[array_index] = left[left_index]
            left_index += 1
        else:
            array[array_index] = right[right_index]
            right_index += 1
        array_index += 1

    while left_index < len(left):
        array[array_index] = left[left_index]
        left_index += 1
        array_index += 1

    while right_index < len(right):
        array[array_index] = right[right_index]
        right_index += 1
        array_index += 1


def _get_min_run_length(n: int) -> int:
    """Calculates the minimum length of a run for the given input size."""
    r = 0
    while n >= MIN_RUN:
        r |= n & 1
        n >>= 1
    return n + r


def tim_sort(
    array: list[T],
    compare_function: CompareFunction[T] | None = None,
    start: int = 0,
    end: int | None = None,
) -> list[T]:
    """
    Implementation of the TimSort algorithm, which combines the best features of
    insertion sort and merge sort. It provides a stable sort with O(n log n)
    worst-case time complexity.

    Args:
        array: Array to sort.
        compare_function: Function to compare elements.
        start: Starting index for the sort range.
        end: Ending index for the sort range.

    Returns:
        Sorted array.

    Example:
        >>> tim_sort([3, 1, 4, 1, 5])
        [1, 1, 3, 4, 5]
        >>> tim_sort(['b', 'a', 'c'])
        ['a', 'b', 'c']
    """
    result = list(array)
    if len(result) <= 1:
        return result

    if compare_function is None:
        compare_function = _compare_function_default

    if end is None:
        end = len(result) - 1

    n = end - start + 1
    min_run = _get_min_run_length(n)

    run_start = start
    while run_start <= end:
        run_end = min(run_start + min_run - 1, end)
        _insertion_sort_range(result, run_start, run_end, compare_function)
        run_start += min_run

    size = min_run
    while size < n:
        left = start
        while left <= end:
            mid = left + size - 1
            right = min(left + 2 * size - 1, end)

            if mid < right:
                _merge(result, left, mid, right, compare_function)
            left += 2 * size
        size *= 2

    return result
