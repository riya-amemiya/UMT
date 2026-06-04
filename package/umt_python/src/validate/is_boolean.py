def is_boolean(value: object) -> bool:
    """
    Determines if the value is a boolean.

    Args:
        value: Value to check.

    Returns:
        True if the value is a boolean, False otherwise.

    Example:
        >>> is_boolean(True)
        True
        >>> is_boolean(False)
        True
        >>> is_boolean(0)
        False
        >>> is_boolean("true")
        False
    """
    return isinstance(value, bool)
