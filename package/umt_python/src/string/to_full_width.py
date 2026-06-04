import re


def to_full_width(string_: str) -> str:
    """
    Convert half-width alphanumeric characters to full-width characters.
    Inverse of :func:`to_half_width`; only digits, uppercase and lowercase
    ASCII letters are converted.

    Args:
        string_: String to convert.

    Returns:
        Converted string.

    Example:
        >>> to_full_width("Abc123")
        'Ａｂｃ１２３'
        >>> to_full_width("a-b!")
        'ａ-ｂ!'
        >>> to_full_width("")
        ''
    """
    return re.sub(
        r"[0-9A-Za-z]",
        lambda match: chr(ord(match.group(0)) + 0xFEE0),
        string_,
    )
