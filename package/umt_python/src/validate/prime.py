from .is_prime_number import is_prime_number
from .number import NumberRule


def prime(message: str = "") -> NumberRule:
    """
    Creates a rule for checking if a number is prime.

    Args:
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``NumberRule`` that passes for prime numbers.

    Example:
        >>> rule = prime()
        >>> rule.validate(7)
        True
        >>> rule.validate(8)
        False
    """
    return NumberRule(validate=is_prime_number, message=message)
