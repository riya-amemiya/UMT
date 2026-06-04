def capitalize(string_: str) -> str:
    """
    Capitalizes the first grapheme of a string. Surrogate-pair safe.
    Does not lowercase the rest of the string.

    Args:
        string_: Input string.

    Returns:
        String with the first grapheme uppercased.

    Example:
        >>> capitalize("hello")
        'Hello'
        >>> capitalize("éclair")
        'Éclair'
        >>> capitalize("")
        ''
        >>> capitalize("hELLO")
        'HELLO'
    """
    for first in string_:
        return first.upper() + string_[len(first) :]
    return string_
