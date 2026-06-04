def ensure_suffix(string_: str, suffix: str) -> str:
    """
    Ensures a string ends with the given suffix, appending it only when it is
    not already present.

    Args:
        string_: Input string.
        suffix: Suffix to ensure.

    Returns:
        String guaranteed to end with the suffix.

    Example:
        >>> ensure_suffix("file", ".txt")
        'file.txt'
        >>> ensure_suffix("file.txt", ".txt")
        'file.txt'
        >>> ensure_suffix("", "/")
        '/'
        >>> ensure_suffix("abc", "")
        'abc'
    """
    return string_ if string_.endswith(suffix) else string_ + suffix
