import unicodedata


def deburr(string_: str) -> str:
    """
    Removes diacritical marks from a string by decomposing characters and
    stripping the combining marks (e.g. "é" becomes "e").

    Args:
        string_: Input string.

    Returns:
        String without diacritical marks.

    Example:
        >>> deburr("déjà vu")
        'deja vu'
        >>> deburr("Crème brûlée")
        'Creme brulee'
        >>> deburr("résumé café")
        'resume cafe'
        >>> deburr("hello world")
        'hello world'
    """
    decomposed = unicodedata.normalize("NFD", string_)
    return "".join(char for char in decomposed if not unicodedata.combining(char))
