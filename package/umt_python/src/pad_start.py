def pad_start(string_: str, target_length: int, pad_string: str) -> str:
    """
    Pads the start of a string with another string until the target length is reached.

    Args:
        string_: The original string to pad.
        target_length: The target length after padding.
        pad_string: The string to use for padding.

    Returns:
        The padded string.

    Raises:
        ValueError: If pad_string is empty.

    Example:
        >>> pad_start("123", 5, "0")
        '00123'
        >>> pad_start("abc", 8, "def")
        'defdeabc'
        >>> pad_start("world", 3, "!") # target_length is less than string length
        'world'
    """
    if not pad_string:
        raise ValueError("pad_string cannot be empty")

    if len(string_) >= target_length:
        return string_

    padding = ""
    padding_length = target_length - len(string_)

    while len(padding) < padding_length:
        padding += pad_string
    
    return padding[:padding_length] + string_
