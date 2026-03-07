import re


def escape_regexp(string: str) -> str:
    """
    Escapes characters in a string for use in a regular expression.

    Args:
        string: The string to escape.

    Returns:
        The escaped string.

    Example:
        >>> escape_regexp("a.b")
        "a\\.b"
    """
    return re.sub(r"[$()*+.?\[\\\]^{|}]", r"\\\g<0>", string)
