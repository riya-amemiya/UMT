def is_prime_number(n: int | float) -> bool:
    """
    Determines if a number is prime.

    Args:
        n: Number to check (must be an integer or a float representing an integer).

    Returns:
        True if the number is prime, False otherwise.

    Example:
        >>> is_prime_number(2)
        True
        >>> is_prime_number(17)
        True
        >>> is_prime_number(4)
        False
        >>> is_prime_number(1)
        False
        >>> is_prime_number(-3)
        False
    """
    if isinstance(n, bool):
        return False

    if isinstance(n, float):
        if not n.is_integer():
            return False
        n = int(n)

    if not isinstance(n, int):
        return False

    if n <= 1:
        return False

    return all(n % i != 0 for i in range(2, int(n**0.5) + 1))
