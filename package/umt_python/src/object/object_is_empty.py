from typing import Any


def object_is_empty(obj: dict[str, Any]) -> bool:
    """
    Checks if an object is empty (has no own properties).

    Args:
        obj: The object to check

    Returns:
        True if the object is empty, False otherwise

    Example:
        >>> object_is_empty({})
        True
        >>> object_is_empty({"a": 1})
        False
    """
    return len(obj) == 0
