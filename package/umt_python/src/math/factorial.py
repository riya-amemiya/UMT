def factorial(x: int) -> int:
    """
    Calculate factorial of a number.

    Args:
        x: Number to calculate factorial for.

    Returns:
        The factorial of x.

    Example:
        >>> factorial(5)
        120
        >>> factorial(0)
        1
    """
    limit = max(1, x)
    result = 1

    for index in range(2, limit + 1):
        result *= index

    return result
