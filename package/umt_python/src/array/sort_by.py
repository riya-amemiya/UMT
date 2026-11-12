from collections.abc import Callable
from typing import TypeVar

from .compare_function_default import compare_function_default
from .tim_sort import tim_sort

T = TypeVar("T")


def _compare_entries(
    a: tuple[list[object], T],
    b: tuple[list[object], T],
) -> int:
    for key_a, key_b in zip(a[0], b[0], strict=True):
        order = compare_function_default(key_a, key_b)
        if order != 0:
            return order
    return 0


def sort_by(array: list[T], *selectors: Callable[[T], object]) -> list[T]:
    """
    Sorts an array by one or more key selectors with a stable TimSort.

    Each selector is called once per element. Later selectors break ties left by
    earlier ones, and elements whose keys are all equal keep their original order.
    Keys are compared with compare_function_default, so incomparable keys such as
    NaN compare as equal.

    Args:
        array: The array to sort (not mutated).
        *selectors: Functions returning the sort keys, in priority order.

    Returns:
        A new sorted list.

    Example:
        >>> sort_by([{"age": 30}, {"age": 20}], lambda user: user["age"])
        [{'age': 20}, {'age': 30}]
        >>> sort_by(["bb", "a", "ab"], len, lambda s: s)
        ['a', 'ab', 'bb']
    """
    entries = [([selector(value) for selector in selectors], value) for value in array]
    return [value for _, value in tim_sort(entries, _compare_entries)]
