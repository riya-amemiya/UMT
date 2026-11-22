def count_occurrences(string_: str, search: str) -> int:
    """
    Counts the number of non-overlapping occurrences of a substring within a
    string. Returns 0 when the search string is empty.

    Args:
        string_: Input string.
        search: Substring to count.

    Returns:
        Number of occurrences.

    Example:
        >>> count_occurrences("ababab", "ab")
        3
        >>> count_occurrences("aaaa", "aa")
        2
        >>> count_occurrences("banana", "a")
        3
        >>> count_occurrences("hello", "")
        0
    """
    return 0 if search == "" else string_.count(search)
