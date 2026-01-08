from typing import Callable, TypeVar

T = TypeVar("T")
K = TypeVar("K", str, int)


def group_by(
    array: list[T],
    iteratee: Callable[[T, int, list[T]], K],
) -> dict[K, list[T]]:
    """
    Groups elements of an array based on a given iteratee function.

    Args:
        array: Array to group.
        iteratee: Function to determine the group key for each element.

    Returns:
        Object with grouped elements.

    Example:
        >>> group_by([6.1, 4.2, 6.3], lambda x, i, arr: int(x))
        {6: [6.1, 6.3], 4: [4.2]}
        >>> group_by(["one", "two", "three"], lambda x, i, arr: len(x))
        {3: ['one', 'two'], 5: ['three']}
        >>> group_by(["apple", "banana", "carrot"], lambda x, i, arr: x[0])
        {'a': ['apple'], 'b': ['banana'], 'c': ['carrot']}
    """
    result: dict[K, list[T]] = {}
    for index, value in enumerate(array):
        key = iteratee(value, index, array)
        if key not in result:
            result[key] = []
        result[key].append(value)
    return result
