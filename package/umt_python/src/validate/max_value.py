from .number import NumberRule


def max_value(maximum: int | float, message: str = "") -> NumberRule:
    """
    Creates a rule for checking if a number is less than or equal to a maximum
    value.

    Args:
        maximum: Maximum allowed value.
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``NumberRule`` that passes for numbers at or below the maximum.

    Example:
        >>> rule = max_value(10)
        >>> rule.validate(10)
        True
        >>> rule.validate(11)
        False
    """
    return NumberRule(validate=lambda value: value <= maximum, message=message)
