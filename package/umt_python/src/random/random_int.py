import math
import random as random_module


def random_int(min_value: float, max_value: float) -> int:
    """
    Returns a random integer in the closed interval [min_value, max_value].

    The lower bound is ceiled and the upper bound is floored before sampling,
    mirroring the reference implementation.

    Args:
        min_value: Inclusive lower bound.
        max_value: Inclusive upper bound.

    Returns:
        A random integer.

    Example:
        >>> random_int(3, 3)
        3
        >>> 1 <= random_int(1, 6) <= 6
        True
        >>> 1 <= random_int(0.4, 5.7) <= 5
        True
    """
    lo = math.ceil(min_value)
    hi = math.floor(max_value)
    return math.floor(random_module.random() * (hi - lo + 1)) + lo
