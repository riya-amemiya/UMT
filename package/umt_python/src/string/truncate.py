def truncate(string_: str, length: int, suffix: str = "...") -> str:
    """
    Truncate a string to a specified length.

    Args:
        string_: The string to truncate.
        length: The maximum length.
        suffix: The suffix to add when truncating (default "...").

    Returns:
        The truncated string.

    Example:
        >>> truncate("Hello World", 5)
        'Hello...'
        >>> truncate("Hello World", 5, "~")
        'Hello~'
        >>> truncate("Hello", 10)
        'Hello'
        >>> truncate("Hello", -1)
        'Hell...'
    """
    if len(string_) <= length:
        return string_

    return string_[:length] + suffix
