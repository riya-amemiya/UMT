import math


def is_perfect_square(n: int) -> bool:
    """
    Determines if a given integer is a perfect square.

    Args:
        n: Integer to check.

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
    """
    if n < 0:
        return False

    sqrt = math.sqrt(n)
    return sqrt == math.floor(sqrt)
