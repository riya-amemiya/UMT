def truncate(string_: str, length: int, suffix: str = "...") -> str:
    """
    Truncate a string to a specified length.

    Args:
        string_: The string to truncate.
        length: The maximum length.
        suffix: The suffix to add when truncating (default "...").

    Returns:
        The truncated string.

    Raises:
        ValueError: If length is negative.

    Example:
        >>> truncate("Hello World", 5)
        'Hello...'
        >>> truncate("Hello World", 5, "~")
        'Hello~'
        >>> truncate("Hello", 10)
        'Hello'
    """
    if length < 0:
        raise ValueError("Length must be non-negative")

    if len(string_) <= length:
        return string_

    return string_[:length] + suffix
