def is_dictionary_object(value: object) -> bool:
    """
    Determines if the value is a dictionary-type object.

    Args:
        value: Value to check.

    Returns:
        True if the value is a dictionary object, False otherwise.

    Example:
        >>> is_dictionary_object({})
        True
        >>> is_dictionary_object([])
        False
    """
    return isinstance(value, dict) and not isinstance(value, list)
