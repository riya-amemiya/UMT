

def object_is_plain(value: object) -> bool:
    """
    Checks if a value is a plain object (a dict in Python).

    In Python, the closest equivalent to a JavaScript plain object is a dictionary.
    Therefore, this function returns True if the value is of type `dict`, and False otherwise.

    Args:
        value (Any): The value to check.

    Returns:
        bool: True if the value is a plain object (dict), False otherwise.

    Examples:
        >>> object_is_plain({})
        True
        >>> object_is_plain({"a": 1})
        True
        >>> object_is_plain([])
        False
        >>> object_is_plain(None)
        False
    """
    return type(value) is dict
