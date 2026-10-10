import re

_NON_ALNUM_FOLLOWED_RE = re.compile(r"[^a-zA-Z0-9]+(.)")
_TRAILING_NON_ALNUM_RE = re.compile(r"[^a-zA-Z0-9]+$")


def _upper_following(match: re.Match[str]) -> str:
    return match.group(1).upper()


def camel_case(string_: str) -> str:
    """
    Converts a string to camelCase.

    Args:
        string_: The string to convert.

    Returns:
        The camelCase string.

    Example:
        >>> camel_case("hello world")
        'helloWorld'
        >>> camel_case("foo-bar-baz")
        'fooBarBaz'
        >>> camel_case("FooBar")
        'fooBar'
    """
    result = _NON_ALNUM_FOLLOWED_RE.sub(_upper_following, string_)
    result = _TRAILING_NON_ALNUM_RE.sub("", result)
    if result:
        result = result[0].lower() + result[1:]
    return result
