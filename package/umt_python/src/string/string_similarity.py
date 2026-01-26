from .levenshtein_distance import levenshtein_distance


def string_similarity(string1: str, string2: str) -> float:
    """
    Calculates the similarity between two strings as a percentage.

    Uses Levenshtein distance normalized by the length of the longer string.

    Args:
        string1: First string to compare.
        string2: Second string to compare.

    Returns:
        Similarity score between 0 (completely different) and 1 (identical).

    Example:
        >>> string_similarity("hello", "hello")
        1.0
        >>> string_similarity("hello", "hallo")
        0.8
        >>> string_similarity("", "abc")
        0.0
    """
    if string1 == string2:
        return 1.0

    if len(string1) == 0 or len(string2) == 0:
        return 0.0

    distance = levenshtein_distance(string1, string2)
    max_length = max(len(string1), len(string2))

    return 1 - distance / max_length
