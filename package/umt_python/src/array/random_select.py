import random
from typing import TypeVar

T = TypeVar("T")


def random_select(
    array: list[T],
    count: int,
    allow_duplicates: bool = False,
) -> list[T]:
    """
    Randomly selects a specified number of elements from an array.

    Args:
        array: Source array.
        count: Number of elements to select.
        allow_duplicates: Whether to allow duplicate selections (default False).

    Returns:
        Array of randomly selected elements.

    Example:
        >>> result = random_select([1, 2, 3, 4, 5], 2)
        >>> len(result)
        2
    """
    if count == 0:
        return []

    if allow_duplicates:
        return random.choices(array, k=count)

    if count >= len(array):
        return random.sample(array, len(array))

    return random.sample(array, count)
