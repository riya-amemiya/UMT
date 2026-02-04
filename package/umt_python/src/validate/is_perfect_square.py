import math


def is_perfect_square(n: int | float) -> bool:
    """
    Determines if a given number is a perfect square.

    Args:
        n: Number to check.

    Returns:
        True if the number is a perfect square, False otherwise.

    Example:
        >>> is_perfect_square(16)
        True
        >>> is_perfect_square(25)
        True
        >>> is_perfect_square(10)
        False
        >>> is_perfect_square(-4)
        False
        >>> is_perfect_square(2.25)
        False
    """
    if isinstance(n, bool):
        return False

    if isinstance(n, float):
        if not n.is_integer():
            return False
        n = int(n)

    if not isinstance(n, int):
        return False

    if n < 0:
        return False

    sqrt = math.isqrt(n)
    return sqrt * sqrt == n
