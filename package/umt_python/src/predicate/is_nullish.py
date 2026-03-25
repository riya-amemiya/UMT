def is_nullish(value: object) -> bool:
    """
    Checks whether a value is None.

    In Python, the equivalent of JavaScript's null/undefined is None.

    Args:
        value: The value to check

    Returns:
        True if the value is None

    Example:
        >>> is_nullish(None)
        True
        >>> is_nullish(0)
        False
        >>> is_nullish("")
        False
    """
    return value is None
