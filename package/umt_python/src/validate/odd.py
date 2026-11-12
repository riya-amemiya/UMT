from .is_double import is_double
from .number import NumberRule


def odd(message: str = "") -> NumberRule:
    """
    Creates a rule for checking if a number is odd.

    Args:
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``NumberRule`` that passes for odd integers.

    Example:
        >>> rule = odd()
        >>> rule.validate(3)
        True
        >>> rule.validate(2)
        False
        >>> rule.validate(1.5)
        False
    """
    return NumberRule(
        validate=lambda value: not is_double(value, False) and value % 2 != 0,
        message=message,
    )
