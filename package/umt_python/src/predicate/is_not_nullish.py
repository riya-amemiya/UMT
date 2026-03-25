def is_not_nullish(value: object) -> bool:
    """
    Checks whether a value is not None.

    In Python, the equivalent of JavaScript's null/undefined is None.

    Args:
        value: The value to check

    Returns:
        True if the value is not None

    Example:
        >>> is_not_nullish(None)
        False
        >>> is_not_nullish(0)
        True
        >>> is_not_nullish("")
        True
    """
    return value is not None
