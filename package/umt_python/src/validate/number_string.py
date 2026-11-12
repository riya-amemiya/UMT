from .is_number import is_number
from .string import StringRule


def number_string(message: str = "") -> StringRule:
    """
    Creates a rule for checking if a string represents a valid number.

    Args:
        message: Custom error message for validation failure (default: empty
            string).

    Returns:
        A ``StringRule`` that passes for strings convertible to a finite number.

    Example:
        >>> rule = number_string()
        >>> rule.validate("12.5")
        True
        >>> rule.validate("abc")
        False
    """
    return StringRule(validate=is_number, message=message)
