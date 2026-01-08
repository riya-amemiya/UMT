def n_pr(n: int, r: int) -> float:
    """
    Calculates permutations (nPr) - number of ways to arrange r items from n items.

    Args:
        n: Total number of items.
        r: Number of items to arrange.

    Returns:
        Number of permutations, or NaN for invalid arguments.

    Example:
        >>> n_pr(5, 2)
        20
        >>> n_pr(5, 0)
        1
    """
    if n < r or n < 0 or r < 0:
        return float("nan")
    if r == 0:
        return 1
    result = 1
    for index in range(r):
        result *= n - index
    return result
