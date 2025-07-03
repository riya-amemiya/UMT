from typing import Callable
from .random_string import random_string, DEFAULT_RANDOM_STRING_CHARS


def random_string_initialization(
    char_pool: str = DEFAULT_RANDOM_STRING_CHARS,
) -> Callable[[int], str]:
    """
    Initializes a function that generates random strings with a specific character pool.

    Args:
        char_pool: String of characters to use for generating random strings.
                   Defaults to alphanumeric characters.

    Returns:
        A function that generates random strings (size: int) -> str.

    Example:
        >>> from .random_string import DEFAULT_RANDOM_STRING_CHARS # For doctest
        >>> custom_random = random_string_initialization("xyz")
        >>> r_string = custom_random(3)
        >>> len(r_string) == 3
        True
        >>> all(c in "xyz" for c in r_string)
        True
        >>> default_pool_func = random_string_initialization()
        >>> another_string = default_pool_func(5)
        >>> len(another_string) == 5
        True
        >>> all(c in DEFAULT_RANDOM_STRING_CHARS for c in another_string)
        True
    """
    return lambda size: random_string(size, char_pool)
