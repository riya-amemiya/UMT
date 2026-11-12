from .number import NumberRule


def between(
    minimum: int | float,
    maximum: int | float,
    message: str = "",
) -> NumberRule:
    """
    Creates a rule for checking if a number is greater than or equal to a
    minimum value and less than or equal to a maximum value.

    Args:
        minimum: Minimum allowed value (inclusive).
        maximum: Maximum allowed value (inclusive).
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``NumberRule`` that passes for numbers inside the inclusive range.

    Example:
        >>> rule = between(1, 10)
        >>> rule.validate(1)
        True
        >>> rule.validate(10)
        True
        >>> rule.validate(10.5)
        False
    """
    return NumberRule(
        validate=lambda value: minimum <= value <= maximum, message=message
    )
