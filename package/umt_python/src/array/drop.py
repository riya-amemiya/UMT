from typing import Literal, TypeVar

T = TypeVar("T")


def drop(
    array: list[T],
    n: int = 1,
    direction: Literal["left", "right", "between"] = "left",
) -> list[T]:
    """
    Returns a new array with n elements removed from the specified direction.

    Args:
        array: The target array.
        n: The number of elements to remove.
        direction: The direction to remove elements from.

    Returns:
        A new array with n elements removed.

    Example:
        >>> drop([1, 2, 3, 4, 5], 2)
        [3, 4, 5]
        >>> drop([1, 2, 3, 4, 5], 2, "left")
        [3, 4, 5]
        >>> drop([1, 2, 3, 4, 5], 2, "right")
        [1, 2, 3]
        >>> drop([1, 2, 3, 4, 5], 1, "between")
        [1, 2, 4, 5]
    """
    if n < 0:
        return list(array)

    if direction == "left":
        return array[n:]
    elif direction == "right":
        return array[: len(array) - n]
    else:
        mid = len(array) // 2
        start = mid - n // 2
        end = mid + (n + 1) // 2
        return array[:start] + array[end:]
