import secrets
import string

DEFAULT_RANDOM_STRING_CHARS = string.ascii_letters + string.digits


def random_string(size: int = 8, char_pool: str = DEFAULT_RANDOM_STRING_CHARS) -> str:
    """
    Generates a cryptographically secure random string.

    Uses the ``secrets`` module for secure randomness, matching the
    behavior of the TypeScript implementation which uses
    ``crypto.getRandomValues()``.

    Args:
        size: Length of the random string. Defaults to 8.
        char_pool: String of characters to use for generating random string.
                   Defaults to alphanumeric characters.

    Returns:
        Random string. An empty string is returned when char_pool is empty.

    Raises:
        ValueError: If size is negative and char_pool is not empty.

    Example:
        >>> len(random_string()) == 8
        True
        >>> len(random_string(10)) == 10
        True
        >>> all(c in 'abc' for c in random_string(5, 'abc'))
        True
        >>> random_string(5, '')
        ''
    """
    if not char_pool:
        return ""
    if size < 0:
        raise ValueError("size must be non-negative")
    return "".join(secrets.choice(char_pool) for _ in range(size))
