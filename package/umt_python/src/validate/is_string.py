from typing import Any


def is_string(value: Any) -> bool:
    """
    Determines if the value is a string.

    Args:
        value: Value to check.

    Returns:
        True if the value is a string, False otherwise.

    Example:
        >>> is_string("test")
        True
        >>> is_string(123)
        False
    """
    return isinstance(value, str)
