def uncapitalize(string_: str) -> str:
    """
    Lowercases the first grapheme of a string. Surrogate-pair safe.

    Args:
        string_: Input string.

    Returns:
        String with the first grapheme lowercased.

    Example:
        >>> uncapitalize("Hello")
        'hello'
        >>> uncapitalize("ÉCLAIR")
        'éCLAIR'
        >>> uncapitalize("")
        ''
    """
    for first in string_:
        return first.lower() + string_[len(first) :]
    return string_
