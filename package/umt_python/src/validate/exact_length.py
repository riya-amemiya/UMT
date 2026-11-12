from .string import StringRule
from .utf16_length import utf16_length


def exact_length(length: int, message: str = "") -> StringRule:
    """
    Creates a rule for checking if a string has an exact length.

    The length is measured in UTF-16 code units, matching JavaScript's
    ``String.prototype.length``.

    Args:
        length: The exact length that the string should have.
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``StringRule`` that passes for strings of exactly the given length.

    Example:
        >>> rule = exact_length(3)
        >>> rule.validate("abc")
        True
        >>> rule.validate("ab")
        False
    """
    return StringRule(
        validate=lambda value: utf16_length(value) == length, message=message
    )
