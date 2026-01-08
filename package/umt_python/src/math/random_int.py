import random as random_module


def random_int(max_value: int, min_value: int = 0) -> int:
    """
    Generates a random integer between min and max (inclusive).

    Args:
        max_value: Maximum value (inclusive).
        min_value: Minimum value (inclusive, default: 0).

    Returns:
        Random integer between min and max.

    Example:
        >>> 0 <= random_int(10) <= 10
        True
        >>> 5 <= random_int(10, 5) <= 10
        True
    """
    return random_module.randint(min_value, max_value)
