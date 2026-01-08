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


def _merge(
    left: list[T],
    right: list[T],
    compare_function: CompareFunction[T],
) -> list[T]:
    """Helper function to merge two sorted arrays."""
    result: list[T] = []
    l_index = 0
    r_index = 0

    while l_index < len(left) and r_index < len(right):
        if compare_function(left[l_index], right[r_index]) <= 0:
            result.append(left[l_index])
            l_index += 1
        else:
            result.append(right[r_index])
            r_index += 1

    result.extend(left[l_index:])
    result.extend(right[r_index:])
    return result


def merge_sort(
    array: list[T],
    compare_function: CompareFunction[T] | None = None,
) -> list[T]:
    """
    Merge sort implementation.

    Args:
        array: Array to sort.
        compare_function: Comparison function.

    Returns:
        Sorted array.

    Example:
        >>> merge_sort([1, 3, 2, 4, 5])
        [1, 2, 3, 4, 5]
        >>> merge_sort([1, 3, 2, 4, 5], lambda a, b: b - a)
        [5, 4, 3, 2, 1]
    """
    if len(array) <= 1:
        return list(array)

    if compare_function is None:
        compare_function = _compare_function_default

    middle = len(array) // 2
    left = array[:middle]
    right = array[middle:]

    return _merge(
        merge_sort(left, compare_function),
        merge_sort(right, compare_function),
        compare_function,
    )
