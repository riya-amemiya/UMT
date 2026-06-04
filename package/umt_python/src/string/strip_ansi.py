import re

# CSI sequences (colors, cursor movement) and OSC sequences. ANSI escape
# sequences are control characters by definition.
_ANSI_PATTERN = re.compile(
    "(?:\x1b\\[|\x9b)[0-?]*[ -/]*[@-~]|\x1b\\][^\x07\x1b]*(?:\x07|\x1b\\\\)"
)


def strip_ansi(string_: str) -> str:
    """
    Removes ANSI escape sequences (e.g. terminal color codes) from a string,
    keeping the visible text. Unlike :func:`sanitize_string`, which strips all
    non-printable characters, this targets only ANSI sequences. Handles CSI
    sequences (colors, cursor movement) and OSC sequences.

    Args:
        string_: Input string.

    Returns:
        String without ANSI escape sequences.

    Example:
        >>> strip_ansi("\\u001b[31mred\\u001b[0m")
        'red'
        >>> strip_ansi("a\\u001b[2Ab")
        'ab'
        >>> strip_ansi("plain text")
        'plain text'
    """
    return _ANSI_PATTERN.sub("", string_)
