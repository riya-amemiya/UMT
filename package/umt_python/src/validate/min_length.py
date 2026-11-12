from .string import StringRule
from .utf16_length import utf16_length


def min_length(minimum: int, message: str = "") -> StringRule:
    """
    Creates a rule for checking if a string's length is greater than or equal
    to a minimum value.

    The length is measured in UTF-16 code units, matching JavaScript's
    ``String.prototype.length``.

    Args:
        minimum: Minimum allowed length of the string.
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``StringRule`` that passes for strings at or above the minimum length.

    Example:
        >>> rule = min_length(3)
        >>> rule.validate("abc")
        True
        >>> rule.validate("ab")
        False
    """
    return StringRule(
        validate=lambda value: utf16_length(value) >= minimum, message=message
    )
