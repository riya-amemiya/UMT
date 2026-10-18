_HALF_WIDTH_TABLE = str.maketrans(
    {i: i - 0xFEE0 for i in range(0xFF01, 0xFF5F)} | {0x3000: 0x0020}
)


def to_half_width(string_: str) -> str:
    """
    Convert full-width characters (zenkaku) to half-width characters (hankaku).
    This implementation specifically targets full-width alphanumeric characters and some symbols.

    Args:
        string_: String to convert.

    Returns:
        Converted string.

    Example:
        >>> to_half_width("Ｈｅｌｌｏ Ｗｏｒｌｄ １２３")
        'Hello World 123'
        >>> to_half_width("ＡＢＣｄｅｆ")
        'ABCdef'
        >>> to_half_width("！＂＃＄％＆＇（）＊＋，－．／") # Full-width symbols
        "!\"#$%&'()*+,-./"
        >>> to_half_width("　") # Full-width space
        ' '
    """
    return string_.translate(_HALF_WIDTH_TABLE)
