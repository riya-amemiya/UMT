import random
from typing import TypeVar

T = TypeVar("T")


def shuffle_2d_array(array: list[list[T]]) -> list[list[T]]:
    """
    Shuffles all elements in a 2D array while maintaining the row lengths.

    Args:
        array: The 2D array to shuffle.

    Returns:
        A new 2D array with shuffled elements.

    Example:
        >>> arr = [[1, 2], [3, 4], [5, 6]]
        >>> result = shuffle_2d_array(arr)
        >>> [len(row) for row in result] == [len(row) for row in arr]
        True
    """
    flat_array: list[T] = []
    for sub_array in array:
        flat_array.extend(sub_array)

    for index in range(len(flat_array) - 1, 0, -1):
        swap_index = random.randint(0, index)
        flat_array[index], flat_array[swap_index] = (
            flat_array[swap_index],
            flat_array[index],
        )

    row_index = 0
    result: list[list[T]] = []
    for sub_array in array:
        new_row = flat_array[row_index : row_index + len(sub_array)]
        result.append(new_row)
        row_index += len(sub_array)

    return result
