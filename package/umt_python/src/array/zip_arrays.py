from typing import Any


def zip_arrays(*arrays: list[Any]) -> list[list[Any]]:
    """
    Creates a new array by combining elements from multiple arrays at corresponding positions.

    Args:
        *arrays: List of arrays to combine.

    Returns:
        New array with combined elements from each input array.

    Example:
        >>> zip_arrays([1, 2], ['a', 'b'])
        [[1, 'a'], [2, 'b']]
        >>> zip_arrays([1, 2, 3], ['a', 'b'])
        [[1, 'a'], [2, 'b']]
    """
    if len(arrays) == 0:
        return []

    length = min(len(arr) for arr in arrays)
    return [[arr[i] for arr in arrays] for i in range(length)]
