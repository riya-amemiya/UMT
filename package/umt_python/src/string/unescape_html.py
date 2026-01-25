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
    r"&(?:amp|lt|gt|quot|#39|#x27|#x2F|#x60|#x3D);|&#(\d*);|&#x([0-9a-fA-F]*);",
)


def _replace_entity(match: re.Match[str]) -> str:
    dec = match.group(1)
    hex_val = match.group(2)

    if dec is not None and dec != "":
        try:
            code_point = int(dec, 10)
            return chr(code_point)
        except (ValueError, OverflowError):
            return match.group(0)

    if hex_val is not None and hex_val != "":
        try:
            code_point = int(hex_val, 16)
            return chr(code_point)
        except (ValueError, OverflowError):
            return match.group(0)

    return HTML_UNESCAPE_MAP.get(match.group(0), match.group(0))


def unescape_html(string_: str) -> str:
    """
    Unescapes HTML entities in a string.

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
