def pad_start(string_: str, target_length: int, pad_string: str) -> str:
    """
    Pads the start of a string with another string until the target length is reached.

    Args:
        string_: The original string to pad.
        target_length: The target length after padding.
        pad_string: The string to use for padding.

    Returns:
        The padded string. The original string is returned unchanged when
        pad_string is empty or the string is already long enough.

    Example:
        >>> pad_start("123", 5, "0")
        '00123'
        >>> pad_start("abc", 8, "def")
        'defdeabc'
        >>> pad_start("world", 3, "!") # target_length is less than string length
        'world'
        >>> pad_start("abc", 5, "")
        'abc'
    """
    if not pad_string or len(string_) >= target_length:
        return string_

    padding_length = target_length - len(string_)
    repeats = -(-padding_length // len(pad_string))

    return (pad_string * repeats)[:padding_length] + string_
