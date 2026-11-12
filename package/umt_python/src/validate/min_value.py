from .number import NumberRule


def min_value(minimum: int | float, message: str = "") -> NumberRule:
    """
    Creates a rule for checking if a number is greater than or equal to a
    minimum value.

    Args:
        minimum: Minimum allowed value.
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``NumberRule`` that passes for numbers at or above the minimum.

    Example:
        >>> rule = min_value(3)
        >>> rule.validate(3)
        True
        >>> rule.validate(2)
        False
    """
    return NumberRule(validate=lambda value: value >= minimum, message=message)
