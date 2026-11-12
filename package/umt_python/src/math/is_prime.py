def is_prime(n: int | float) -> bool:
    """
    Determines whether a number is prime using 6k +/- 1 trial division.

    Args:
        n: Number to check.

    Returns:
        True if n is a prime integer; False for booleans, non-integers, NaN,
        infinities, and numbers below 2.

    Example:
        >>> is_prime(2)
        True
        >>> is_prime(97)
        True
        >>> is_prime(91)
        False
        >>> is_prime(7.5)
        False
    """
    if isinstance(n, bool):
        return False
    if isinstance(n, float) and not n.is_integer():
        return False
    if n < 2:
        return False
    if n < 4:
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False

    number = int(n)
    divisor = 5
    while divisor * divisor <= number:
        if number % divisor == 0 or number % (divisor + 2) == 0:
            return False
        divisor += 6
    return True
