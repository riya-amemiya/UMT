from collections.abc import Callable
from typing import TypeVar

T = TypeVar("T")


def pick_by(
    object_: dict[str, T],
    predicate: Callable[[T, str], bool],
) -> dict[str, T]:
    """
    Creates a new object containing only entries for which the predicate
    returns True.

    The input object is not mutated.

    Args:
        object_ (dict[str, T]): Source object.
        predicate (Callable[[T, str], bool]): Selector predicate. Receives
            (value, key); entries where this returns True are kept.

    Returns:
        dict[str, T]: A new object with the filtered entries.

    Example:
        >>> pick_by({"a": 1, "b": 2, "c": 3}, lambda v, k: v > 1)
        {'b': 2, 'c': 3}
        >>> pick_by({"a": 1, "b": 2}, lambda v, k: k == "a")
        {'a': 1}
    """
    return {key: value for key, value in object_.items() if predicate(value, key)}
