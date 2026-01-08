from .division import division
from .gcd import gcd
from .multiplication import multiplication
from .value_swap import value_swap


def lcm(x: float, y: float) -> float:
    """
    Least Common Multiple (LCM).

    Args:
        x: First number.
        y: Second number.

    Returns:
        The LCM of x and y.

    Example:
        >>> lcm(2, 3)
        6
        >>> lcm(4, 6)
        12
    """
    if x == 0 or y == 0:
        return 0
    new_x, new_y = value_swap(abs(x), abs(y))
    return multiplication(division(new_x, gcd(new_x, new_y)), new_y)
