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
    # Mapping for full-width to half-width for ASCII range based on Unicode block
    # Full-width exclamation mark to tilde: U+FF01 to U+FF5E
    # Half-width corresponding characters: U+0021 to U+007E
    # The difference is 0xFEE0 (65248)
    # Full-width space U+3000 -> Half-width space U+0020
    translation_table = {}
    for i in range(0xFF01, 0xFF5F): # Covers full-width '！' to '～'
        translation_table[i] = i - 0xFEE0
    translation_table[0x3000] = 0x0020 # Full-width space to half-width space
    
    return string_.translate(translation_table)
