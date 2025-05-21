from .trim_start_characters import trim_start_characters
from .trim_end_characters import trim_end_characters

def trim_characters(string_: str, chars: str) -> str:
    """
    Removes specified characters from both ends of a string.

    Args:
        string_: The string to trim.
        chars: The set of characters to remove.

    Returns:
        A new string with specified characters removed from both ends.

    Example:
        >>> trim_characters("-.-hello-.-", "-.")
        'hello'
        >>> trim_characters("123abc123", "123")
        'abc'
    """
    return trim_end_characters(trim_start_characters(string_, chars), chars)
