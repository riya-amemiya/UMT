def multiples(x: float, n: int) -> list[float]:
    """
    Generate an array of multiples of a number.

    Args:
        x: Base number.
        n: Number of multiples to generate.

    Returns:
        Array of multiples.

    Example:
        >>> multiples(2, 5)
        [2, 4, 6, 8, 10]
    """
    return [x * index for index in range(1, n + 1)]
