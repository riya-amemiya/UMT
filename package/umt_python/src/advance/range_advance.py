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
        start: Starting number.
        end: Ending number (exclusive).
        conditional_expression: Function that determines which numbers to include.

    Returns:
        Array of numbers that satisfy the conditional expression.

    Example:
        >>> range_advance(1, 10, lambda number: number % 2 == 0)
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

    return range_array(actual_start, actual_end)
