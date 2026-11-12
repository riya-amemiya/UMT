import math

from .number import NumberRule


def _is_integer(value: float) -> bool:
    if isinstance(value, int):
        return True
    return math.isfinite(value) and value.is_integer()


def integer(message: str = "") -> NumberRule:
    """
    Creates a rule for checking if a number is an integer.

    Mirrors JavaScript's ``Number.isInteger``: integral floats such as ``1.0``
    pass, while fractional values, ``NaN``, and infinities fail.

    Args:
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``NumberRule`` that passes for integral numbers.

    Example:
        >>> rule = integer()
        >>> rule.validate(3)
        True
        >>> rule.validate(3.0)
        True
        >>> rule.validate(3.5)
        False
        >>> rule.validate(float("inf"))
        False
    """
    return NumberRule(validate=_is_integer, message=message)
