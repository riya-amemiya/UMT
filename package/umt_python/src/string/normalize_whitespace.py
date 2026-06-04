import re


def normalize_whitespace(string_: str) -> str:
    """
    Collapses consecutive whitespace characters into a single space and trims
    the result. Unlike :func:`delete_spaces`, which removes all whitespace,
    this keeps words separated by single spaces.

    Args:
        string_: Input string.

    Returns:
        String with normalized whitespace.

    Example:
        >>> normalize_whitespace("  hello   world \\t\\n foo ")
        'hello world foo'
        >>> normalize_whitespace("a b c")
        'a b c'
        >>> normalize_whitespace("")
        ''
    """
    return re.sub(r"\s+", " ", string_).strip()
