from collections.abc import Sequence


def unzip(array: Sequence[Sequence[object]]) -> list[list[object]]:
    """
    Splits a list of tuples into one list per tuple position (the inverse of zip_arrays).

    The number of result lists is the shortest tuple length, matching the
    truncation of zip_arrays.

    Args:
        array: Tuples to split.

    Returns:
        One list per tuple position.

    Example:
        >>> unzip([[1, "a"], [2, "b"]])
        [[1, 2], ['a', 'b']]
        >>> unzip([(1, "a", True), (2, "b")])
        [[1, 2], ['a', 'b']]
    """
    if len(array) == 0:
        return []
    width = min(len(row) for row in array)
    return [[row[position] for row in array] for position in range(width)]
