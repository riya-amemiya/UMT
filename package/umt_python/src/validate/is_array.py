from typing import Any


def is_array(value: Any) -> bool:
    """
    Determines if the value is a list.

    Args:
        value: Value to check.

    Returns:
        True if the value is a list, False otherwise.

    Example:
        >>> is_array([1, 2, 3])
        True
        >>> is_array({})
        False
    """
    return isinstance(value, list)
