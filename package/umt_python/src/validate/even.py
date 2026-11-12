from .is_double import is_double
from .number import NumberRule


def even(message: str = "") -> NumberRule:
    """
    Creates a rule for checking if a number is even.

    Args:
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``NumberRule`` that passes for even integers.

    Example:
        >>> rule = even()
        >>> rule.validate(2)
        True
        >>> rule.validate(3)
        False
        >>> rule.validate(2.5)
        False
    """
    return NumberRule(
        validate=lambda value: not is_double(value, False) and value % 2 == 0,
        message=message,
    )
