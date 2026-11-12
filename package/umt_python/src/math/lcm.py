from .division import division
from .gcd import gcd
from .multiplication import multiplication
from .value_swap import value_swap


def lcm(x: float, y: float, *z: float) -> float:
    """
    Least Common Multiple (LCM).

    Args:
        x: First number.
        y: Second number.
        *z: Additional numbers (optional).

    Returns:
        The LCM of all input numbers, or 0 if any of them is 0.

    Example:
        >>> lcm(2, 3)
        6
        >>> lcm(4, 6)
        12
        >>> lcm(2, 3, 4)
        12
    """
    result = _lcm_pair(x, y)
    for value in z:
        result = _lcm_pair(result, value)
    return result


def _lcm_pair(x: float, y: float) -> float:
    """Computes the LCM of two numbers."""
    if x == 0 or y == 0:
        return 0
    new_x, new_y = value_swap(abs(x), abs(y))
    return multiplication(division(new_x, gcd(new_x, new_y)), new_y)
