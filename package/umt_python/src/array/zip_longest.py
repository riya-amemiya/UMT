from typing import Any


def zip_longest(*arrays: list[Any]) -> list[list[Any]]:
    """
    Combines arrays of different lengths by padding shorter arrays with None values
    to match the length of the longest array.

    Args:
        *arrays: List of arrays to combine.

    Returns:
        New array with combined elements from each input array,
        padded with None values where necessary.

    Example:
        >>> zip_longest([1, 2], ['a'])
        [[1, 'a'], [2, None]]
        >>> zip_longest([1], ['a', 'b'])
        [[1, 'a'], [None, 'b']]
    """
    if len(arrays) == 0:
        return []

    max_length = max(len(arr) for arr in arrays)
    return [
        [arr[i] if i < len(arr) else None for arr in arrays] for i in range(max_length)
    ]
