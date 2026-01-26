from typing import TypeVar

T = TypeVar("T")


def compact(array: list[T]) -> list[T]:
    """
    Creates an array with all falsey values removed.

    Args:
        array: The array to compact.

    Returns:
        Returns the new array of filtered values.

    Example:
        >>> compact([0, 1, False, 2, '', 3])
        [1, 2, 3]
        >>> compact([None, 0, False, ''])
        []
    """
    return [item for item in array if item]
