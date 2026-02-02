def levenshtein_distance(string1: str, string2: str) -> int:
    """
    Calculates the Levenshtein distance between two strings.

    Returns the minimum number of single-character edits (insertions, deletions,
    or substitutions) required to change one string into the other.

    Args:
        string1: First string to compare.
        string2: Second string to compare.

    Returns:
        The Levenshtein distance.

    Example:
        >>> levenshtein_distance("kitten", "sitting")
        3
        >>> levenshtein_distance("hello", "hello")
        0
        >>> levenshtein_distance("", "abc")
        3
    """
    if len(string1) < len(string2):
        string1, string2 = string2, string1

    length1 = len(string1)
    length2 = len(string2)

    if length2 == 0:
        return length1

    previous_row: list[int] = list(range(length2 + 1))
    current_row: list[int] = [0] * (length2 + 1)

    for i, char1 in enumerate(string1):
        current_row[0] = i + 1
        for j, char2 in enumerate(string2):
            cost = 0 if char1 == char2 else 1
            current_row[j + 1] = min(
                previous_row[j + 1] + 1,
                current_row[j] + 1,
                previous_row[j] + cost,
            )
        previous_row, current_row = current_row, previous_row

    return previous_row[length2]
