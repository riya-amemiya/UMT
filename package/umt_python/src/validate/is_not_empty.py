def is_not_empty(obj: object) -> bool:
    """
    Checks if an object is not empty.

    Mirrors the TypeScript ``Object.keys(object).length > 0`` check: non-empty
    dictionaries, lists, and strings are not empty, while numbers, booleans,
    sets, and other values have no enumerable keys and are reported as empty.

    Args:
        obj: The object to check.

    Returns:
        True if the object is not empty, False if it is empty.

    Raises:
        TypeError: If ``obj`` is ``None``, matching ``Object.keys(null)`` and
            ``Object.keys(undefined)``.

    Example:
        >>> is_not_empty({})
        False
        >>> is_not_empty({"a": 1})
        True
        >>> is_not_empty([1])
        True
        >>> is_not_empty("hello")
        True
        >>> is_not_empty(1)
        False
    """
    if obj is None:
        raise TypeError("Cannot convert undefined or null to object")
    if isinstance(obj, (dict, list, str)):
        return len(obj) > 0
    return False
