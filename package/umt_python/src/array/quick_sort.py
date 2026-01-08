from typing import Callable, TypeVar

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
) -> T:
    """Finds the median value among three elements in the array."""
    ab = compare_function(array[a], array[b])
    if ab < 0:
        bc = compare_function(array[b], array[c])
        if bc < 0:
            return array[b]
        if compare_function(array[a], array[c]) < 0:
            return array[c]
        return array[a]
    ac = compare_function(array[a], array[c])
    if ac < 0:
        return array[a]
    if compare_function(array[b], array[c]) < 0:
        return array[c]
    return array[b]


def _partition(
    array: list[T],
    low: int,
    high: int,
    compare_function: CompareFunction[T],
) -> int:
    """Partitions the array around a pivot element."""
    pivot = _median_of_three(array, low, (low + high) // 2, high, compare_function)
    left = low - 1
    right = high + 1

    while True:
        left += 1
        while compare_function(array[left], pivot) < 0:
            left += 1

        right -= 1
        while compare_function(array[right], pivot) > 0:
            right -= 1

        if left >= right:
            return right

        array[left], array[right] = array[right], array[left]


def _sort_impl(
    array: list[T],
    low_init: int,
    high_init: int,
    compare_function: CompareFunction[T],
    insertion_sort_threshold: int,
) -> None:
    """Internal implementation of the quicksort algorithm."""
    low = low_init
    high = high_init

    while low < high:
        if high - low < insertion_sort_threshold:
            _insertion_sort_range(array, low, high, compare_function)
            return

        pivot_index = _partition(array, low, high, compare_function)
        if pivot_index - low < high - pivot_index:
            _sort_impl(
                array, low, pivot_index, compare_function, insertion_sort_threshold
            )
            low = pivot_index + 1
        else:
            _sort_impl(
                array, pivot_index + 1, high, compare_function, insertion_sort_threshold
            )
            high = pivot_index


def quick_sort(
    array: list[T],
    compare_function: CompareFunction[T] | None = None,
    start_index: int = 0,
    end_index: int | None = None,
    insertion_sort_threshold: int = 10,
) -> list[T]:
    """
    Sorts an array using a hybrid algorithm combining QuickSort and InsertionSort.

    Args:
        array: Array to sort.
        compare_function: Comparison function that returns negative if a < b,
                          zero if a = b, positive if a > b.
        start_index: Starting index for the sort range (default 0).
        end_index: Ending index for the sort range (default array.length - 1).
        insertion_sort_threshold: Threshold for switching to insertion sort (default 10).

    Returns:
        Sorted array.

    Example:
        >>> quick_sort([1, 3, 2, 4, 5])
        [1, 2, 3, 4, 5]
        >>> quick_sort([1, 3, 2], lambda a, b: b - a)
        [3, 2, 1]
        >>> quick_sort(['b', 'a', 'c'])
        ['a', 'b', 'c']
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
        _sort_impl(
            result, valid_start, valid_end, compare_function, insertion_sort_threshold
        )

    return result
