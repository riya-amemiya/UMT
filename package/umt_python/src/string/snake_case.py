from .words import words


def snake_case(string_: str) -> str:
    """
    Converts a string to snake_case.

    Args:
        string_: Input string.

    Returns:
        snake_case string.

    Example:
        >>> snake_case("helloWorld")
        'hello_world'
        >>> snake_case("Hello World")
        'hello_world'
        >>> snake_case("FooBar")
        'foo_bar'
        >>> snake_case("XMLHttpRequest")
        'xml_http_request'
    """
    return "_".join(word.lower() for word in words(string_))
