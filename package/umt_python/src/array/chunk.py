from typing import TypeVar

T = TypeVar("T")


def chunk(array: list[T], n: int) -> list[list[T]]:
    """
    Split an array into smaller chunks of specified size.

    Args:
        array: The array to split.
        n: The size of each chunk.

    Returns:
        Array of chunks.

    Example:
        >>> chunk([1, 2, 3, 4, 5, 6, 7, 8, 9], 3)
        [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        >>> chunk([1, 2, 3, 4, 5], 2)
        [[1, 2], [3, 4], [5]]
    """
    result: list[list[T]] = []
    for i in range(0, len(array), n):
        result.append(array[i : i + n])
    return result
