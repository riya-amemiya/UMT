from collections.abc import Callable
from typing import TypeVar

T = TypeVar("T")


def max_by(array: list[T], selector: Callable[[T], object]) -> T | None:
    """
    Returns the element whose selected value is the largest.

    Values are compared with ``>``, so the first element wins ties and a key that
    never compares as larger (such as NaN) never replaces the current maximum.

    Args:
        array: The array to scan.
        selector: Function returning the value to compare.

    Returns:
        The element with the largest selected value, or None for an empty array.

    Example:
        >>> max_by([{"n": 1}, {"n": 2}], lambda item: item["n"])
        {'n': 2}
        >>> max_by([], lambda item: item) is None
        True
    """
    if len(array) == 0:
        return None
    best = array[0]
    best_key = selector(best)
    for value in array[1:]:
        key = selector(value)
        if key > best_key:  # type: ignore[operator]
            best = value
            best_key = key
    return best
