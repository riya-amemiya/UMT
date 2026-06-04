def remove_suffix(string_: str, suffix: str) -> str:
    """
    Removes the given suffix from the end of a string once, if present.
    Unlike :func:`trim_end_characters`, which removes any of a set of
    characters repeatedly, this removes the exact suffix a single time.

    Args:
        string_: Input string.
        suffix: Suffix to remove.

    Returns:
        String without the trailing suffix.

    Example:
        >>> remove_suffix("file.txt", ".txt")
        'file'
        >>> remove_suffix("file", ".txt")
        'file'
        >>> remove_suffix("path//", "/")
        'path/'
        >>> remove_suffix("abc", "")
        'abc'
    """
    if suffix != "" and string_.endswith(suffix):
        return string_[: -len(suffix)]
    return string_
