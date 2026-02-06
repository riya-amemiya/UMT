from collections.abc import Callable
from typing import TypeVar

from src.array.quick_sort import quick_sort

T = TypeVar("T")
CompareFunction = Callable[[T, T], int | float]


def quick_sort_simple(
    array: list[T],
    compare_function: CompareFunction[T] | None = None,
    start_id: int = 0,
    end_id: int | None = None,
) -> list[T]:
    """
    Quick sort implementation for arrays with simple validation logic.

    Args:
        array: Input array to sort.
        compare_function: Function to determine sort order.
        start_id: Starting index for sorting (default 0).
        end_id: Ending index for sorting (default array.length - 1).

    Returns:
        Sorted array.
    """
    local_start_id = start_id

    local_end_id = len(array) - 1 if end_id is None else end_id

    if (
        local_start_id < 0
        or local_start_id >= len(array)
        or local_start_id > local_end_id
    ):
        local_start_id = 0

    if local_end_id < 0 or local_end_id >= len(array):
        local_end_id = len(array) - 1

    return quick_sort(array, compare_function, local_start_id, local_end_id)
