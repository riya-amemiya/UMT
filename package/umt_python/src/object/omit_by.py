from collections.abc import Callable
from typing import TypeVar

T = TypeVar("T")


def omit_by(
    object_: dict[str, T],
    predicate: Callable[[T, str], bool],
) -> dict[str, T]:
    """
    Creates a new object containing only entries for which the predicate
    returns False.

    The input object is not mutated.

    Args:
        object_ (dict[str, T]): Source object.
        predicate (Callable[[T, str], bool]): Rejection predicate. Receives
            (value, key); entries where this returns True are removed.

    Returns:
        dict[str, T]: A new object with the remaining entries.

    Example:
        >>> omit_by({"a": 1, "b": None, "c": 3}, lambda v, k: v is None)
        {'a': 1, 'c': 3}
        >>> omit_by({"a": 1, "b": 2}, lambda v, k: k == "a")
        {'b': 2}
    """
    return {key: value for key, value in object_.items() if not predicate(value, key)}
