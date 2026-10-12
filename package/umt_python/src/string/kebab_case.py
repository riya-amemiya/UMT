import re

_LOWER_UPPER_RE = re.compile(r"([a-z])([A-Z])")
_ACRONYM_RE = re.compile(r"([A-Z])([A-Z][a-z])")
_SPACE_UNDERSCORE_RE = re.compile(r"[\s_]+")
_NON_ALNUM_RE = re.compile(r"[^a-zA-Z0-9-]")
_MULTI_DASH_RE = re.compile(r"-+")
_EDGE_DASH_RE = re.compile(r"^-|-$")


def kebab_case(string_: str) -> str:
    """
    Converts a string to kebab-case.

    Args:
        string_: The string to convert.

    Returns:
        The kebab-case string.

    Example:
        >>> kebab_case("helloWorld")
        'hello-world'
        >>> kebab_case("FooBar")
        'foo-bar'
        >>> kebab_case("foo_bar_baz")
        'foo-bar-baz'
    """
    result = _LOWER_UPPER_RE.sub(r"\1-\2", string_)
    result = _ACRONYM_RE.sub(r"\1-\2", result)
    result = _SPACE_UNDERSCORE_RE.sub("-", result)
    result = _NON_ALNUM_RE.sub("-", result)
    result = _MULTI_DASH_RE.sub("-", result)
    result = _EDGE_DASH_RE.sub("", result)
    return result.lower()
