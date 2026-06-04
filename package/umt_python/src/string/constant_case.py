from .words import words


def constant_case(string_: str) -> str:
    """
    Converts a string to CONSTANT_CASE.

    Args:
        string_: Input string.

    Returns:
        CONSTANT_CASE string.

    Example:
        >>> constant_case("helloWorld")
        'HELLO_WORLD'
        >>> constant_case("Hello World")
        'HELLO_WORLD'
        >>> constant_case("foo-bar-baz")
        'FOO_BAR_BAZ'
        >>> constant_case("")
        ''
    """
    return "_".join(word.upper() for word in words(string_))
