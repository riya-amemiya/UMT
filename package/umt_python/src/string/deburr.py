import re
import unicodedata

_DIACRITIC_RE = re.compile(
    "["
    r"`^\u00a8\u00af\u00b4\u00b7-\u00b8\u02b0-\u034e"
    r"\u0350-\u0357\u035d-\u0362\u0374-\u0375\u037a\u0384-\u0385"
    r"\u0483-\u0487\u0559\u0591-\u05bd\u05bf\u05c1-\u05c2\u05c4-\u05c5"
    r"\u05c7\u064b-\u0652\u0657-\u0658\u06df-\u06e0\u06e5-\u06e6"
    r"\u06ea-\u06ec\u0730-\u074a\u07a6-\u07b0\u07eb-\u07f5\u0818-\u0819"
    r"\u0898-\u089f\u08c9-\u08d2\u08e3-\u08fe\u093c\u094d\u0951-\u0954"
    r"\u0971\u09bc\u09cd\u0a3c\u0a4d\u0abc\u0acd\u0afd-\u0aff\u0b3c"
    r"\u0b4d\u0b55\u0bcd\u0c3c\u0c4d\u0cbc\u0ccd\u0d3b-\u0d3c\u0d4d"
    r"\u0dca\u0e3a\u0e47-\u0e4c\u0e4e\u0eba\u0ec8-\u0ecc\u0f18-\u0f19"
    r"\u0f35\u0f37\u0f39\u0f3e-\u0f3f\u0f82-\u0f84\u0f86-\u0f87\u0fc6"
    r"\u1037\u1039-\u103a\u1063-\u1064\u1069-\u106d\u1087-\u108d\u108f"
    r"\u109a-\u109b\u135d-\u135f\u1714-\u1715\u1734\u17c9-\u17d3\u17dd"
    r"\u1939-\u193b\u1a60\u1a75-\u1a7c\u1a7f\u1ab0-\u1abe\u1ac1-\u1acb"
    r"\u1acf-\u1add\u1ae0-\u1aeb\u1b34\u1b44\u1b6b-\u1b73\u1baa-\u1bab"
    r"\u1be6\u1bf2-\u1bf3\u1c36-\u1c37\u1c78-\u1c7d\u1cd0-\u1ce8\u1ced"
    r"\u1cf4\u1cf7-\u1cf9\u1d2c-\u1d6a\u1d9b-\u1dbe\u1dc4-\u1dcf"
    r"\u1df5-\u1dff\u1fbd\u1fbf-\u1fc1\u1fcd-\u1fcf\u1fdd-\u1fdf"
    r"\u1fed-\u1fef\u1ffd-\u1ffe\u2cef-\u2cf1\u2e2f\u302a-\u302f"
    r"\u3099-\u309c\u30fc\ua66f\ua67c-\ua67d\ua67f\ua69c-\ua69d"
    r"\ua6f0-\ua6f1\ua700-\ua721\ua788-\ua78a\ua7f1\ua7f8-\ua7f9\ua806"
    r"\ua82c\ua8c4\ua8e0-\ua8f1\ua92b-\ua92e\ua953\ua9b3\ua9c0\ua9e5"
    r"\uaa7b-\uaa7d\uaabf-\uaac2\uaaf6\uab5b-\uab5f\uab69-\uab6b"
    r"\uabec-\uabed\ufb1e\ufe20-\ufe2f\uff3e\uff40\uff70\uff9e-\uff9f"
    r"\uffe3\U000102e0\U00010780-\U00010785\U00010787-\U000107b0"
    r"\U000107b2-\U000107ba\U00010a38-\U00010a3a\U00010a3f"
    r"\U00010ae5-\U00010ae6\U00010d22-\U00010d27\U00010d4e"
    r"\U00010d69-\U00010d6d\U00010efa\U00010efd-\U00010eff"
    r"\U00010f46-\U00010f50\U00010f82-\U00010f85\U00011046\U00011070"
    r"\U000110b9-\U000110ba\U00011133-\U00011134\U00011173\U000111c0"
    r"\U000111ca-\U000111cc\U00011235-\U00011236\U000112e9-\U000112ea"
    r"\U0001133b-\U0001133c\U0001134d\U00011366-\U0001136c"
    r"\U00011370-\U00011374\U000113ce-\U000113d0\U000113d2-\U000113d3"
    r"\U000113e1-\U000113e2\U00011442\U00011446\U000114c2-\U000114c3"
    r"\U000115bf-\U000115c0\U0001163f\U000116b6-\U000116b7\U0001172b"
    r"\U00011839-\U0001183a\U0001193d-\U0001193e\U00011943\U000119e0"
    r"\U00011a34\U00011a47\U00011a99\U00011c3f\U00011d42"
    r"\U00011d44-\U00011d45\U00011d97\U00011dd9\U00011f41-\U00011f42"
    r"\U00011f5a\U00013447-\U00013455\U0001612f\U00016af0-\U00016af4"
    r"\U00016b30-\U00016b36\U00016d6b-\U00016d6c\U00016f8f-\U00016f9f"
    r"\U00016ff0-\U00016ff1\U0001aff0-\U0001aff3\U0001aff5-\U0001affb"
    r"\U0001affd-\U0001affe\U0001cf00-\U0001cf2d\U0001cf30-\U0001cf46"
    r"\U0001d167-\U0001d169\U0001d16d-\U0001d172\U0001d17b-\U0001d182"
    r"\U0001d185-\U0001d18b\U0001d1aa-\U0001d1ad\U0001e030-\U0001e06d"
    r"\U0001e130-\U0001e136\U0001e2ae\U0001e2ec-\U0001e2ef"
    r"\U0001e5ee-\U0001e5ef\U0001e8d0-\U0001e8d6\U0001e944-\U0001e946"
    r"\U0001e948-\U0001e94a"
    "]"
)


def deburr(string_: str) -> str:
    """
    Removes diacritical marks from a string by decomposing characters and
    stripping every character with the Unicode ``Diacritic`` property
    (e.g. "é" becomes "e").

    Args:
        string_: Input string.

    Returns:
        String without diacritical marks.

    Example:
        >>> deburr("déjà vu")
        'deja vu'
        >>> deburr("Crème brûlée")
        'Creme brulee'
        >>> deburr("résumé café")
        'resume cafe'
        >>> deburr("hello world")
        'hello world'
    """
    return _DIACRITIC_RE.sub("", unicodedata.normalize("NFD", string_))
