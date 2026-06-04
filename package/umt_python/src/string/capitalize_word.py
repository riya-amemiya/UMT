def capitalize_word(word: str) -> str:
    """
    Uppercases the first character of a word and lowercases the rest.

    Args:
        word: Input word.

    Returns:
        The normalized word with its first character uppercased and the
        remaining characters lowercased.

    Example:
        >>> capitalize_word("hello")
        'Hello'
        >>> capitalize_word("hELLO")
        'Hello'
        >>> capitalize_word("")
        ''
        >>> capitalize_word("a")
        'A'
    """
    return word[:1].upper() + word[1:].lower()
