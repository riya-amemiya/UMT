import math


def truncate_middle(string_: str, length: int, separator: str = "...") -> str:
    """
    Truncate a string in the middle, keeping its head and tail.

    The result length (in code points, like ``truncate``) is at most
    ``length``. When ``length`` is not larger than the separator, the
    separator is cut to ``length``.

    Args:
        string_: The string to truncate.
        length: The maximum length of the result.
        separator: The text placed between head and tail (default "...").

    Returns:
        The truncated string.

    Example:
        >>> truncate_middle("Hello World", 8)
        'Hel...ld'
        >>> truncate_middle("Hello World", 7, "~")
        'Hel~rld'
        >>> truncate_middle("Hello", 10)
        'Hello'
        >>> truncate_middle("Hello World", 2)
        '..'
    """
    if len(string_) <= length:
        return string_
    available = length - len(separator)
    if available <= 0:
        return separator[: max(0, length)]
    head_length = math.ceil(available / 2)
    tail_length = available - head_length
    return string_[:head_length] + separator + string_[len(string_) - tail_length :]
