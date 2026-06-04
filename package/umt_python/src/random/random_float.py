import random as random_module


def random_float(min_value: float, max_value: float) -> float:
    """
    Returns a random float in the half-open interval [min_value, max_value).

    Args:
        min_value: Inclusive lower bound.
        max_value: Exclusive upper bound.

    Returns:
        A random float.

    Example:
        >>> random_float(7, 7)
        7.0
        >>> 0 <= random_float(0, 1) < 1
        True
    """
    return random_module.random() * (max_value - min_value) + min_value
