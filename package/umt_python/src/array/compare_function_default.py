from typing import TypeVar

T = TypeVar("T")


def compare_function_default(a: T, b: T) -> int:
    """
    Default comparison function that returns 1 if a > b, -1 if a < b, 0 if a equals b.

    Args:
        a: First value to compare.
        b: Second value to compare.

    Returns:
        Comparison result (-1, 0, or 1).

    Example:
        >>> compare_function_default(2, 1)
        1
        >>> compare_function_default(1, 2)
        -1
        >>> compare_function_default(2, 2)
        0
        >>> compare_function_default("b", "a")
        1
        >>> compare_function_default("a", "b")
        -1
    """
    if a > b:  # type: ignore
        return 1
    if a < b:  # type: ignore
        return -1
    return 0
