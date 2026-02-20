from collections.abc import Iterable


def sum_precise(numbers: Iterable[int | float]) -> float:
    """
    Computes the sum of an array of numbers using the Neumaier
    summation algorithm for improved floating-point precision.

    Args:
        numbers: The array of numbers to sum.

    Returns:
        The precise sum of all numbers.

    Example:
        >>> sum_precise([0.1, 0.2, 0.3])
        0.6
        >>> sum_precise([1e20, 1, -1e20])
        1.0
    """
    sum_ = 0.0
    compensation = 0.0

    for number in numbers:
        t = sum_ + number
        if abs(sum_) >= abs(number):
            compensation += (sum_ - t) + number
        else:
            compensation += (number - t) + sum_
        sum_ = t

    return sum_ + compensation
