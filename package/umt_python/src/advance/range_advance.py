from collections.abc import Callable

from src.array.range_array import range_array


def range_advance(
    start: int | float,
    end: int | float | None = None,
    conditional_expression: Callable[[int | float], bool] | None = None,
) -> list[int | float]:
    """
    Returns an array of numbers that satisfy the conditional expression.

    Args:
        start: Starting number (or ending number if end is None)
        end: Ending number (exclusive, defaults to None)
        conditional_expression: Function that determines which numbers to include

    Returns:
        Array of numbers that satisfy the conditional expression

    Example:
        >>> range_advance(1, 10, lambda x: x % 2 == 0)
        [2, 4, 6, 8]
    """
    actual_start = 0 if end is None else start
    actual_end = start if end is None else end

    if conditional_expression is not None:
        array: list[int | float] = []
        index = actual_start
        while index < actual_end:
            if conditional_expression(index):
                array.append(index)
            index += 1
        return array

    if end is None:
        return range_array(0, start)
    return range_array(start, end)
