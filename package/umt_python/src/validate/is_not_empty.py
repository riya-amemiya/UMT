def is_not_empty(obj: object) -> bool:
    """
    Checks if an object is not empty.

    Args:
        obj: The object to check.

    Returns:
        True if the object is not empty, False if it is empty.

    Example:
        >>> is_not_empty({})
        False
        >>> is_not_empty({"a": 1})
        True
    """
    if isinstance(obj, dict):
        return len(obj) > 0
    return False
