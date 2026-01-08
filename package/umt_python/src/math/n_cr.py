
from .n_pr import n_pr


def n_cr(n: int, r: int) -> float:
    """
    Calculates combinations (nCr) - number of ways to choose r items from n items.

    Args:
        n: Total number of items.
        r: Number of items to choose.

    Returns:
        Number of combinations, or NaN for invalid arguments.

    Example:
        >>> n_cr(5, 2)
        10
        >>> n_cr(5, 0)
        1
    """
    if n < r or n < 0 or r < 0:
        return float("nan")
    if r == 0 or n == r:
        return 1

    numerator = n_pr(n, r)
    denominator = 1

    for index in range(2, r + 1):
        denominator *= index

    return numerator / denominator
