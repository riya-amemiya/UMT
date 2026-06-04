from .capitalize_word import capitalize_word
from .words import words


def pascal_case(string_: str) -> str:
    """
    Converts a string to PascalCase.

    Args:
        string_: Input string.

    Returns:
        PascalCase string.

    Example:
        >>> pascal_case("hello-world")
        'HelloWorld'
        >>> pascal_case("hello_world")
        'HelloWorld'
        >>> pascal_case("helloWorld")
        'HelloWorld'
        >>> pascal_case("XMLHttpRequest")
        'XmlHttpRequest'
    """
    return "".join(capitalize_word(word) for word in words(string_))
