from .capitalize_word import capitalize_word
from .words import words


def title_case(string_: str) -> str:
    """
    Converts a string to Title Case (each word capitalized, separated by
    spaces).

    Args:
        string_: Input string.

    Returns:
        Title Case string.

    Example:
        >>> title_case("hello world")
        'Hello World'
        >>> title_case("a-quick brown_fox")
        'A Quick Brown Fox'
        >>> title_case("helloWorld")
        'Hello World'
    """
    return " ".join(capitalize_word(word) for word in words(string_))
