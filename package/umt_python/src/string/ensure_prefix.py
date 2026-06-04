def ensure_prefix(string_: str, prefix: str) -> str:
    """
    Ensures a string starts with the given prefix, prepending it only when it
    is not already present.

    Args:
        string_: Input string.
        prefix: Prefix to ensure.

    Returns:
        String guaranteed to start with the prefix.

    Example:
        >>> ensure_prefix("example.com", "https://")
        'https://example.com'
        >>> ensure_prefix("https://example.com", "https://")
        'https://example.com'
        >>> ensure_prefix("", "/")
        '/'
        >>> ensure_prefix("abc", "")
        'abc'
    """
    return string_ if string_.startswith(prefix) else prefix + string_
