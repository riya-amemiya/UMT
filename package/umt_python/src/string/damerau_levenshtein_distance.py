def damerau_levenshtein_distance(string1: str, string2: str) -> int:
    """
    Calculates the optimal string alignment (restricted Damerau-Levenshtein)
    distance between two strings.

    Insertions, deletions, substitutions and transpositions of two adjacent
    characters each count as one edit, and no substring is edited twice.
    Characters are compared as code points, like ``levenshtein_distance``.
    Uses three rows sized by the shorter string, so space is O(min(N, M)).

    Args:
        string1: First string to compare.
        string2: Second string to compare.

    Returns:
        The optimal string alignment distance.

    Example:
        >>> damerau_levenshtein_distance("ca", "ac")
        1
        >>> damerau_levenshtein_distance("kitten", "sitting")
        3
        >>> damerau_levenshtein_distance("ca", "abc")
        3
    """
    if string1 == string2:
        return 0
    if len(string1) <= len(string2):
        short, long = string1, string2
    else:
        short, long = string2, string1
    short_length = len(short)
    long_length = len(long)

    if short_length == 0:
        return long_length

    rows: list[list[int]] = [
        list(range(short_length + 1)),
        [0] * (short_length + 1),
        [0] * (short_length + 1),
    ]

    for row_index in range(1, long_length + 1):
        current = rows[row_index % 3]
        previous = rows[(row_index + 2) % 3]
        before_previous = rows[(row_index + 1) % 3]
        long_char = long[row_index - 1]
        current[0] = row_index
        for col_index in range(1, short_length + 1):
            short_char = short[col_index - 1]
            cost = 0 if short_char == long_char else 1
            best = min(
                previous[col_index] + 1,
                current[col_index - 1] + 1,
                previous[col_index - 1] + cost,
            )
            if (
                row_index > 1
                and col_index > 1
                and long_char == short[col_index - 2]
                and long[row_index - 2] == short_char
            ):
                best = min(best, before_previous[col_index - 2] + 1)
            current[col_index] = best

    return rows[long_length % 3][short_length]
