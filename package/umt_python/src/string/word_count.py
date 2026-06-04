from .words import words


def word_count(string_: str) -> int:
    """
    Counts words in a string using the same boundaries as :func:`words`.

    Args:
        string_: Input string.

    Returns:
        Word count.

    Example:
        >>> word_count("hello world")
        2
        >>> word_count("camelCaseSplit")
        3
        >>> word_count("")
        0
        >>> word_count("foo, bar! baz?")
        3
    """
    return len(words(string_))
