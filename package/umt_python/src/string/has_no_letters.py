def has_no_letters(text: str) -> bool:
    """
    Checks if the string contains no letters (contains only emojis, numbers, or special characters).

    Args:
        text: The string to check.

    Returns:
        True if the string has no letters, False otherwise.

    Example:
        >>> has_no_letters("123")
        True
        >>> has_no_letters("🌟123#")
        True
        >>> has_no_letters("abc123")
        False
    """
    for char_code in text:
        if char_code.isalpha():
            return False
    return True
