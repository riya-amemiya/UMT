from .string import StringRule
from .utf16_length import utf16_length


def max_length(maximum: int, message: str = "") -> StringRule:
    """
    Creates a rule for checking if a string's length is less than or equal to
    a maximum value.

    The length is measured in UTF-16 code units, matching JavaScript's
    ``String.prototype.length``.

    Args:
        maximum: Maximum allowed length of the string.
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``StringRule`` that passes for strings at or below the maximum length.

    Example:
        >>> rule = max_length(3)
        >>> rule.validate("abc")
        True
        >>> rule.validate("abcd")
        False
    """
    return StringRule(
        validate=lambda value: utf16_length(value) <= maximum, message=message
    )
