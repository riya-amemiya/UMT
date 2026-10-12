import re
import unicodedata

_DIACRITICS_RE = re.compile(r"[\u0300-\u036f]")
_NON_WORD_RE = re.compile(r"[^\w\s-]")
_WHITESPACE_RE = re.compile(r"\s+")
_UNDERSCORE_RE = re.compile(r"_+")
_MULTI_DASH_RE = re.compile(r"-+")
_EDGE_DASH_RE = re.compile(r"^-+|-+$")


def slugify(string_: str) -> str:
    """
    Convert a string to a URL-friendly slug.

    Args:
        string_: The string to convert.

    Returns:
        The slugified string.

    Example:
        >>> slugify("Hello World!")
        'hello-world'
        >>> slugify("This is a Test")
        'this-is-a-test'
        >>> slugify("Café")
        'cafe'
    """
    result = unicodedata.normalize("NFD", string_)
    result = _DIACRITICS_RE.sub("", result)
    result = result.lower()
    result = _NON_WORD_RE.sub("-", result)
    result = _WHITESPACE_RE.sub("-", result)
    result = _UNDERSCORE_RE.sub("-", result)
    result = _MULTI_DASH_RE.sub("-", result)
    return _EDGE_DASH_RE.sub("", result)
