from .n_cr import n_cr


def n_hr(n: int, r: int) -> float:
    """
    Calculates combinations with repetition (nHr) - ways to choose r items from n items with repetition allowed.

    Args:
        n: Total number of items.
        r: Number of items to choose.

    Returns:
        Number of combinations with repetition, or NaN for invalid arguments.

    Example:
        >>> n_hr(5, 2)
        15
    """
    if n == 0 or r == 0 or n < 0 or r < 0:
        return float("nan")

    return n_cr(n + r - 1, r)
