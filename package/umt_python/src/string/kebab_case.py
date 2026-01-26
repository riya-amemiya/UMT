import re


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
    result = re.sub(r"([a-z])([A-Z])", r"\1-\2", string_)
    result = re.sub(r"([A-Z])([A-Z][a-z])", r"\1-\2", result)
    result = re.sub(r"[\s_]+", "-", result)
    result = re.sub(r"[^a-zA-Z0-9-]", "-", result)
    result = re.sub(r"-+", "-", result)
    result = re.sub(r"^-|-$", "", result)
    return result.lower()
