import math


def is_prime_number(n: int) -> bool:
    """
    Determines if a number is prime.

    Args:
        n: Number to check (must be an integer).

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
    if n <= 1 or not isinstance(n, int):
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False

    return all(n % index != 0 for index in range(3, int(math.sqrt(n)) + 1, 2))
