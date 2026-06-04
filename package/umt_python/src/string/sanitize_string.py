import re


def sanitize_string(string_: str) -> str:
    """
    Sanitizes a string by removing non-printable characters.

    This is useful for cleaning up output from external commands before
    displaying it in the terminal. It allows printable ASCII characters
    (space to tilde), newlines, carriage returns, and tabs.

    Args:
        string_: The string to sanitize.

    Returns:
        The sanitized string.

    Example:
        >>> sanitize_string("Hello, World!")
        'Hello, World!'
        >>> sanitize_string("a\\u0000b")
        'ab'
        >>> sanitize_string("héllo")
        'hllo'
        >>> sanitize_string("日本語")
        ''
    """
    return re.sub(r"[^\t\n\r -~]", "", string_)
