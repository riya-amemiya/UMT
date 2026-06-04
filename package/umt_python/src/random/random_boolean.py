import random as random_module


def random_boolean(probability: float = 0.5) -> bool:
    """
    Returns a random boolean. The probability of True defaults to 0.5.

    Args:
        probability: Probability of returning True (0..1).

    Returns:
        A random boolean.

    Example:
        >>> isinstance(random_boolean(), bool)
        True
        >>> random_boolean(1)
        True
        >>> random_boolean(0)
        False
    """
    return random_module.random() < probability
