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


def _median_of_three(
    array: list[T],
    a: int,
    b: int,
    c: int,
    compare_function: CompareFunction[T],
) -> int:
    """Get the median of three elements in the array."""
    values = [
        {"index": a, "value": array[a]},
        {"index": b, "value": array[b]},
        {"index": c, "value": array[c]},
    ]
    values.sort(key=lambda x: x["value"], reverse=False)
    sorted_values = sorted(
        values, key=lambda x: (compare_function(x["value"], values[0]["value"]),)
    )
    return sorted_values[1]["index"]


def _partition(
    array: list[T],
    low: int,
    high: int,
    compare_function: CompareFunction[T],
) -> tuple[int, int]:
    """Select dual pivots and partition the array into three parts."""
    length = high - low
    gap = max(1, length // 3)

    left_pivot_index = _median_of_three(
        array, low, low + gap, min(low + 2 * gap, high), compare_function
    )
    right_pivot_index = _median_of_three(
        array, max(low, high - 2 * gap), high - gap, high, compare_function
    )

    if left_pivot_index != low:
        array[low], array[left_pivot_index] = array[left_pivot_index], array[low]
    if right_pivot_index != high:
        array[high], array[right_pivot_index] = array[right_pivot_index], array[high]

    if compare_function(array[low], array[high]) > 0:
        array[low], array[high] = array[high], array[low]

    left = low + 1
    right = high - 1
    current = left

    while current <= right:
        if compare_function(array[current], array[low]) < 0:
            array[current], array[left] = array[left], array[current]
            left += 1
        elif compare_function(array[current], array[high]) > 0:
            while current < right and compare_function(array[right], array[high]) > 0:
                right -= 1
            array[current], array[right] = array[right], array[current]
            right -= 1
            if compare_function(array[current], array[low]) < 0:
                array[current], array[left] = array[left], array[current]
                left += 1
        current += 1

    left -= 1
    right += 1
    array[low], array[left] = array[left], array[low]
    array[high], array[right] = array[right], array[high]

    return left, right


def _sort_range(
    array: list[T],
    start: int,
    end: int,
    compare_function: CompareFunction[T],
    insertion_sort_threshold: int,
) -> None:
    """Internal implementation of dual-pivot quicksort."""
    if end - start < insertion_sort_threshold:
        if end > start:
            _insertion_sort_range(array, start, end, compare_function)
        return

    left_pivot_index, right_pivot_index = _partition(
        array, start, end, compare_function
    )

    _sort_range(
        array, start, left_pivot_index - 1, compare_function, insertion_sort_threshold
    )

    if right_pivot_index - left_pivot_index > 1:
        _sort_range(
            array,
            left_pivot_index + 1,
            right_pivot_index - 1,
            compare_function,
            insertion_sort_threshold,
        )

    _sort_range(
        array, right_pivot_index + 1, end, compare_function, insertion_sort_threshold
    )


def dual_pivot_quick_sort(
    array: list[T],
    compare_function: CompareFunction[T] | None = None,
    start_index: int = 0,
    end_index: int | None = None,
    insertion_sort_threshold: int = 10,
) -> list[T]:
    """
    Sort array using dual-pivot quicksort algorithm.
    More efficient than traditional quicksort for arrays with many duplicate values.

    Args:
        array: Array to be sorted.
        compare_function: Optional comparison function.
        start_index: Optional starting index (default 0).
        end_index: Optional ending index (default array length - 1).
        insertion_sort_threshold: Optional threshold for insertion sort (default 10).

    Returns:
        Sorted array.

    Example:
        >>> dual_pivot_quick_sort([3, 1, 4, 1, 5, 9, 2, 6, 5, 3])
        [1, 1, 2, 3, 3, 4, 5, 5, 6, 9]
        >>> dual_pivot_quick_sort(['banana', 'apple', 'orange'])
        ['apple', 'banana', 'orange']
    """
    result = list(array)
    if len(result) <= 1:
        return result

    if compare_function is None:
        compare_function = _compare_function_default

    if end_index is None:
        end_index = len(result) - 1

    valid_start = max(0, min(start_index, len(result) - 1))
    valid_end = max(0, min(end_index, len(result) - 1))

    if valid_start < valid_end:
        _sort_range(
            result, valid_start, valid_end, compare_function, insertion_sort_threshold
        )

    return result
