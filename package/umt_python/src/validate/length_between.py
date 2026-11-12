from .string import StringRule
from .utf16_length import utf16_length


def length_between(minimum: int, maximum: int, message: str = "") -> StringRule:
    """
    Creates a rule for checking if a string's length is greater than or equal
    to a minimum value and less than or equal to a maximum value.

    The length is measured in UTF-16 code units, matching JavaScript's
    ``String.prototype.length``.

    Args:
        minimum: Minimum allowed length of the string (inclusive).
        maximum: Maximum allowed length of the string (inclusive).
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``StringRule`` that passes for strings whose length is inside the
        inclusive range.

    Example:
        >>> rule = length_between(2, 4)
        >>> rule.validate("ab")
        True
        >>> rule.validate("abcd")
        True
        >>> rule.validate("abcde")
        False
    """
    return StringRule(
        validate=lambda value: minimum <= utf16_length(value) <= maximum,
        message=message,
    )
