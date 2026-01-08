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

    for index in range(2, int(math.sqrt(n)) + 1):
        if n % index == 0:
            return False

    return True
