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
    result: list[T] = []
    used_indices: set[int] = set()

    while len(result) < count and (allow_duplicates or len(result) < len(array)):
        random_index = random.randint(0, len(array) - 1)
        if allow_duplicates or random_index not in used_indices:
            used_indices.add(random_index)
            result.append(array[random_index])

    return result
