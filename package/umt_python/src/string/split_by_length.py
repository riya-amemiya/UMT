def split_by_length(string_: str, length: int) -> list[str]:
    """
    Splits a string into chunks of the specified length. Surrogate-pair safe,
    so characters outside the Basic Multilingual Plane are not split.

    Args:
        string_: Input string.
        length: Maximum length of each chunk.

    Returns:
        List of string chunks.

    Example:
        >>> split_by_length("abcdefg", 3)
        ['abc', 'def', 'g']
        >>> split_by_length("abcdef", 2)
        ['ab', 'cd', 'ef']
        >>> split_by_length("", 3)
        []
    """
    chars = list(string_)
    return [
        "".join(chars[index : index + length]) for index in range(0, len(chars), length)
    ]
