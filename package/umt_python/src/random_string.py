import random
import string

DEFAULT_RANDOM_STRING_CHARS = string.ascii_letters + string.digits


def random_string(size: int = 8, char_pool: str = DEFAULT_RANDOM_STRING_CHARS) -> str:
    """
    Generates a random string.

    Args:
        size: Length of the random string. Defaults to 8.
        char_pool: String of characters to use for generating random string.
                   Defaults to alphanumeric characters.

    Returns:
        Random string.
    
    Raises:
        ValueError: If size is negative or char_pool is empty (when size > 0).

    Example:
        >>> len(random_string()) == 8
        True
        >>> len(random_string(10)) == 10
        True
        >>> all(c in 'abc' for c in random_string(5, 'abc'))
        True
    """
    if size < 0:
        raise ValueError("size must be non-negative")
    if size == 0:
        return ""
    if not char_pool:
        raise ValueError("char_pool cannot be empty")
    return "".join(random.choice(char_pool) for _ in range(size))
