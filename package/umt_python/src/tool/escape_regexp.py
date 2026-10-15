import re

_REGEXP_SPECIAL_RE = re.compile(r"[$()*+.?\[\\\]^{|}]")


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
    return _REGEXP_SPECIAL_RE.sub(r"\\\g<0>", string)
