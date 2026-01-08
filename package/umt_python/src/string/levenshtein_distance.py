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
    length1 = len(string1)
    length2 = len(string2)

    if length1 == 0:
        return length2
    if length2 == 0:
        return length1

    matrix: list[list[int]] = [[0] * (length2 + 1) for _ in range(length1 + 1)]

    for i in range(length1 + 1):
        matrix[i][0] = i
    for j in range(length2 + 1):
        matrix[0][j] = j

    for i in range(1, length1 + 1):
        for j in range(1, length2 + 1):
            cost = 0 if string1[i - 1] == string2[j - 1] else 1
            matrix[i][j] = min(
                matrix[i - 1][j] + 1,
                matrix[i][j - 1] + 1,
                matrix[i - 1][j - 1] + cost,
            )

    return matrix[length1][length2]
