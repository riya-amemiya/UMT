_ASCII_ALNUM = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
_FULL_WIDTH_TABLE = str.maketrans(
    _ASCII_ALNUM, "".join(chr(ord(character) + 0xFEE0) for character in _ASCII_ALNUM)
)


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
    return string_.translate(_FULL_WIDTH_TABLE)
