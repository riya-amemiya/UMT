def factorize(n: int) -> list[int]:
    """
    Prime factorization of a number.

    Args:
        n: Number to factorize.

    Returns:
        Array of prime factors.

    Example:
        >>> factorize(12)
        [2, 2, 3]
    """
    result: list[int] = []
    remaining = abs(n)

    factor = 2
    while factor * factor <= remaining:
        while remaining % factor == 0:
            result.append(factor)
            remaining //= factor
        factor += 1

    if remaining > 1:
        result.append(remaining)

    return result
