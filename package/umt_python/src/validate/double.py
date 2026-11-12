from .is_double import is_double
from .number import NumberRule


def double(message: str = "") -> NumberRule:
    """
    Creates a rule for checking if a number is a floating point value.

    Args:
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``NumberRule`` that passes for numbers with a fractional part.

    Example:
        >>> rule = double()
        >>> rule.validate(1.5)
        True
        >>> rule.validate(1)
        False
    """
    return NumberRule(validate=is_double, message=message)
