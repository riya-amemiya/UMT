def reverse_string(char: str) -> str:
    """
    Reverses a string.

    Args:
        char: String to reverse.

    Returns:
        Reversed string.

    Example:
        >>> reverse_string("Hello")
        'olleH'
        >>> reverse_string("madam")
        'madam'
    """
    return char[::-1]
