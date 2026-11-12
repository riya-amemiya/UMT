from .capitalize import capitalize
from .words import words


def sentence_case(string_: str) -> str:
    """
    Converts a string to Sentence case.

    The first word is capitalized, every other word is lower case, and all
    words are separated by single spaces. Words are split with ``words``.

    Args:
        string_: Input string.

    Returns:
        Sentence case string.

    Example:
        >>> sentence_case("helloWorld foo-bar")
        'Hello world foo bar'
        >>> sentence_case("XMLHttpRequest")
        'Xml http request'
        >>> sentence_case("")
        ''
    """
    return " ".join(
        capitalize(word.lower()) if index == 0 else word.lower()
        for index, word in enumerate(words(string_))
    )
