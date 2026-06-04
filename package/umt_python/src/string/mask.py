def mask(string_: str, start: int = 1, end: int = 1, char: str = "*") -> str:
    """
    Masks the middle portion of a string with a fill character, preserving the
    leading and trailing visible counts. Surrogate-pair safe.

    Args:
        string_: Input string.
        start: Number of leading characters to keep visible. Defaults to 1.
        end: Number of trailing characters to keep visible. Defaults to 1.
        char: Mask character. Defaults to "*".

    Returns:
        Masked string.

    Example:
        >>> mask("1234567890", start=2, end=4)
        '12****7890'
        >>> mask("secret")
        's****t'
        >>> mask("secret", char="#")
        's####t'
        >>> mask("abc", start=2, end=1)
        'abc'
    """
    graphemes = list(string_)
    length = len(graphemes)
    if start + end >= length:
        return string_
    middle_length = length - start - end
    return (
        "".join(graphemes[:start])
        + char * middle_length
        + "".join(graphemes[length - end :])
    )
