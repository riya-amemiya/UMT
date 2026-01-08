def trim_start_characters(string_: str, chars: str) -> str:
    """
    Removes specified characters from the start of a string.

    Args:
        string_: The input string to trim.
        chars: Characters to remove from the start.

    Returns:
        A new string with specified characters removed from the start.

    Example:
        >>> trim_start_characters("!!!hello", "!")
        'hello'
        >>> trim_start_characters("---123", "-")
        '123'
        >>> trim_start_characters("abc123", "xyz")
        'abc123'
    """
    start_index = 0
    while start_index < len(string_) and string_[start_index] in chars:
        start_index += 1
    return string_[start_index:]
