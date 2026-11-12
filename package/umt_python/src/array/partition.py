from collections.abc import Callable
from typing import TypeVar

T = TypeVar("T")


def partition(
    array: list[T],
    predicate: Callable[[T, int, list[T]], bool],
) -> tuple[list[T], list[T]]:
    """
    Splits an array into two arrays based on a predicate.

    Args:
        array: The array to partition.
        predicate: Function returning True for elements that go into the first array.

    Returns:
        A tuple of (pass, fail) arrays.

    Example:
        >>> partition([1, 2, 3, 4], lambda n, i, arr: n % 2 == 0)
        ([2, 4], [1, 3])
    """
    passed: list[T] = []
    failed: list[T] = []
    for index, value in enumerate(array):
        if predicate(value, index, array):
            passed.append(value)
        else:
            failed.append(value)
    return passed, failed
