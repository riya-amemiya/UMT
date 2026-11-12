import math


def quotient(x: float, y: float) -> list[float]:
    """
    Computes quotient and remainder of division.

    Args:
        x: Dividend.
        y: Divisor.

    Returns:
        Array containing [quotient, remainder].

    Example:
        >>> quotient(5, 2)
        [2.0, 1.0]
    """
    if y == 0 or math.isinf(x):
        return [float("nan"), float("nan")]
    remainder = math.fmod(x, y)
    return [(x - remainder) / y, remainder + 0]
