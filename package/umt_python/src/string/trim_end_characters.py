def trim_end_characters(string_: str, chars: str) -> str:
    """
    Removes specified characters from the end of a string.

    Args:
        string_: The input string to trim.
        chars: Characters to remove from the end.

    Returns:
        A new string with specified characters removed from the end.

    Example:
        >>> trim_end_characters("hello!!!", "!")
        'hello'
        >>> trim_end_characters("123---", "-")
        '123'
        >>> trim_end_characters("abc123", "xyz")
        'abc123'
    """
    return string_.rstrip(chars) if chars else string_
