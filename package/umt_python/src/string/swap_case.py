import unicodedata


def swap_case(string_: str) -> str:
    """
    Swaps the case of each letter in a string, turning uppercase letters into
    lowercase and vice versa. Characters without case are left unchanged.

    Args:
        string_: Input string.

    Returns:
        String with swapped case.

    Example:
        >>> swap_case("Hello World")
        'hELLO wORLD'
        >>> swap_case("ABC")
        'abc'
        >>> swap_case("abc123-日本")
        'ABC123-日本'
        >>> swap_case("")
        ''
    """

    def swap(char: str) -> str:
        if not unicodedata.category(char).startswith("L"):
            return char
        return char.upper() if char == char.lower() else char.lower()

    return "".join(swap(char) for char in string_)
