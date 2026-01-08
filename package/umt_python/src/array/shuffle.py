import random
from typing import TypeVar

T = TypeVar("T")


def shuffle(array: list[T]) -> list[T]:
    """
    Randomly shuffles the elements of an array.

    Args:
        array: Array to shuffle.

    Returns:
        New array with shuffled elements.

    Example:
        >>> arr = [1, 2, 3, 4, 5]
        >>> result = shuffle(arr)
        >>> len(result) == len(arr)
        True
        >>> set(result) == set(arr)
        True
    """
    shuffled_array = list(array)
    for index in range(len(shuffled_array) - 1, 0, -1):
        swap_index = random.randint(0, index)
        if index != swap_index:
            shuffled_array[index], shuffled_array[swap_index] = (
                shuffled_array[swap_index],
                shuffled_array[index],
            )
    return shuffled_array
