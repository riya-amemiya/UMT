import re

def delete_spaces(string_: str) -> str:
    """
    Removes all whitespace characters from a string.

    Args:
        string_: Input string.

    Returns:
        String with all whitespace characters removed.

    Example:
        >>> delete_spaces("Hello World")
        'HelloWorld'
        >>> delete_spaces("  tab\\t space ")
        'tabspace'
    """
    return re.sub(r'\s', '', string_)
