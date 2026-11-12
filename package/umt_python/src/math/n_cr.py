import sys


def n_cr(n: int, r: int) -> float:
    """
    Calculates combinations (nCr) - number of ways to choose r items from n items.

    Uses the multiplicative formula with k = min(r, n - r), so the result stays
    finite whenever it fits in a float.

    Args:
        n: Total number of items.
        r: Number of items to choose.

    Returns:
        Number of combinations, inf when the result exceeds the largest float,
        or NaN for invalid arguments.

    Example:
        >>> n_cr(5, 2)
        10.0
        >>> n_cr(5, 0)
        1
        >>> n_cr(1000, 500)
        2.7028824094543666e+299
    """
    if n < r or n < 0 or r < 0:
        return float("nan")
    if r in (0, n):
        return 1

    effective_r = min(r, n - r)
    base = n - effective_r

    result = 1.0
    for index in range(1, effective_r + 1):
        factor = base + index
        result = (
            result / index * factor
            if result > sys.float_info.max / factor
            else result * factor / index
        )
        if result == float("inf"):
            return result

    return result
