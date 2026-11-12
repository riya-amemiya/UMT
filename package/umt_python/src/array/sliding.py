from typing import TypeVar

T = TypeVar("T")


def sliding(array: list[T], size: int, step: int = 1) -> list[list[T]]:
    """
    Returns overlapping windows of a fixed size from an array.

    Args:
        array: The array to window.
        size: Window size.
        step: Step between window starts (default 1).

    Returns:
        Array of windows (incomplete trailing windows are omitted).

    Example:
        >>> sliding([1, 2, 3, 4, 5], 3)
        [[1, 2, 3], [2, 3, 4], [3, 4, 5]]
        >>> sliding([1, 2, 3, 4, 5], 3, 2)
        [[1, 2, 3], [3, 4, 5]]
    """
    return [
        array[index : index + size] for index in range(0, len(array) - size + 1, step)
    ]
