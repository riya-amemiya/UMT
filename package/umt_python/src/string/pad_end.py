def pad_end(string_: str, target_length: int, pad_string: str) -> str:
    """
    Adds the specified string to the end of the string until it reaches the specified length.

    Args:
        string_: The original string to apply padding.
        target_length: The target length after padding.
        pad_string: The string to use for padding.

    Returns:
        The string after padding has been applied.

    Example:
        >>> pad_end("abc", 5, "0")
        'abc00'
        >>> pad_end("hello", 8, "123")
        'hello123'
        >>> pad_end("world", 3, "!") # target_length is less than string length
        'world'
        >>> pad_end("test", 6, "") # pad_string is empty
        'test'
    """
    if not pad_string:
        return string_
    result = string_
    while len(result) < target_length:
        needed = target_length - len(result)
        result += pad_string[:needed]
    return result
