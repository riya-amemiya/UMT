def quotient(x: float, y: float) -> list[float]:
    """
    Computes quotient and remainder of division.

    Args:
        x: Dividend.
        y: Divisor.

    Returns:
        Array containing [quotient, remainder].

    Example:
        >>> quotient(5, 2)
        [2.0, 1.0]
    """
    return [(x - (x % y)) / y, (x % y) + 0]
