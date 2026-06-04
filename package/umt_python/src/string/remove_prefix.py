def remove_prefix(string_: str, prefix: str) -> str:
    """
    Removes the given prefix from the start of a string once, if present.
    Unlike :func:`trim_start_characters`, which removes any of a set of
    characters repeatedly, this removes the exact prefix a single time.

    Args:
        string_: Input string.
        prefix: Prefix to remove.

    Returns:
        String without the leading prefix.

    Example:
        >>> remove_prefix("https://example.com", "https://")
        'example.com'
        >>> remove_prefix("example.com", "https://")
        'example.com'
        >>> remove_prefix("//path", "/")
        '/path'
        >>> remove_prefix("abc", "")
        'abc'
    """
    if prefix != "" and string_.startswith(prefix):
        return string_[len(prefix) :]
    return string_
