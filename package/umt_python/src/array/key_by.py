from collections.abc import Callable
from typing import TypeVar

T = TypeVar("T")


def key_by(
    collection: list[T] | dict[str, T],
    iteratee: Callable[[T], str] | str | None = None,
) -> dict[str, T]:
    """
    Creates an object composed of keys generated from the results of running each element
    of collection through iteratee.

    Args:
        collection: The collection to iterate over (list or dict)
        iteratee: The iteratee function or property name to generate the key

    Returns:
        A dictionary keyed by the iteratee result

    Example:
        >>> key_by([{"id": "a"}, {"id": "b"}], "id")
        {'a': {'id': 'a'}, 'b': {'id': 'b'}}
        >>> key_by([1, 2, 3])
        {'1': 1, '2': 2, '3': 3}
    """

    def get_key(value: T) -> str:
        if iteratee is None:
            return str(value)
        if callable(iteratee):
            return str(iteratee(value))
        return (
            str(value[iteratee])
            if isinstance(value, dict)
            else str(getattr(value, iteratee))
        )

    result: dict[str, T] = {}

    if isinstance(collection, list):
        for value in collection:
            key = get_key(value)
            result[key] = value
    else:
        for value in collection.values():
            key = get_key(value)
            result[key] = value

    return result
