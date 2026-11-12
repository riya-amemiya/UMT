import re

HTML_UNESCAPE_MAP: dict[str, str] = {
    "&amp;": "&",
    "&lt;": "<",
    "&gt;": ">",
    "&quot;": '"',
    "&#39;": "'",
    "&#x27;": "'",
    "&#x2F;": "/",
    "&#x60;": "`",
    "&#x3D;": "=",
}

ENTITY_REGEX = re.compile(
    r"&(?:amp|lt|gt|quot|#39|#x27|#x2F|#x60|#x3D);"
    r"|&#([0-9]{1,7});|&#x([0-9A-Fa-f]{1,6});",
)


def _is_safe_code_point(code_point: int) -> bool:
    return not (
        code_point in {0, 0x7F}
        or code_point > 0x10FFFF
        or 0xD800 <= code_point <= 0xDFFF
        or (code_point not in (0x09, 0x0A, 0x0D) and code_point <= 0x1F)
        or 0x80 <= code_point <= 0x9F
    )


def _replace_entity(match: re.Match[str]) -> str:
    dec = match.group(1)
    hex_val = match.group(2)

    if dec is not None:
        code_point = int(dec, 10)
        return chr(code_point) if _is_safe_code_point(code_point) else match.group(0)

    if hex_val is not None:
        code_point = int(hex_val, 16)
        return chr(code_point) if _is_safe_code_point(code_point) else match.group(0)

    return HTML_UNESCAPE_MAP[match.group(0)]


def unescape_html(string_: str) -> str:
    """
    Unescapes HTML entities in a string.

    Numeric references that resolve to NULL, C0 controls other than TAB / LF /
    CR, DEL, C1 controls, surrogates, or values above U+10FFFF are left
    unchanged.

    Args:
        string_: The string to unescape.

    Returns:
        The unescaped string with HTML entities converted back to their
        original characters.

    Example:
        >>> unescape_html("&lt;script&gt;alert(&quot;Hello&quot;);&lt;/script&gt;")
        '<script>alert("Hello");</script>'
        >>> unescape_html("Tom &amp; Jerry")
        'Tom & Jerry'
        >>> unescape_html("5 &lt; 10 &amp;&amp; 10 &gt; 5")
        '5 < 10 && 10 > 5'
    """
    return ENTITY_REGEX.sub(_replace_entity, string_)
