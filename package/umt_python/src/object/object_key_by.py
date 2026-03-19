from collections.abc import Callable
from typing import Any, TypeVar, cast

T = TypeVar("T")
PropertyName = str | int
IterateeFunction = Callable[[T], PropertyName]
Iteratee = IterateeFunction[T] | str | int | None

def _normalize_iteratee(iteratee: Iteratee[T]) -> IterateeFunction[T]:
    if iteratee is None:
        return lambda value: cast(PropertyName, value)
    if callable(iteratee):
        return iteratee
    return lambda obj: cast(PropertyName, cast(Any, obj)[iteratee])

def object_key_by(
    collection: list[T] | dict[Any, T],
    iteratee: Iteratee[T] = None,
) -> dict[PropertyName, T]:
    """
    Creates an object composed of keys generated from the results of running each element of collection through iteratee.

    Args:
        collection: The collection to iterate over
        iteratee: The iteratee function or property name to generate the key

    Returns:
        dict[PropertyName, T]: Returns the composed aggregate object.
    """
    get_key = _normalize_iteratee(iteratee)
    result: dict[PropertyName, T] = {}

    if isinstance(collection, list):
        for value in collection:
            key = get_key(value)
            result[key] = value
        return result

    for value in collection.values():
        key = get_key(value)
        result[key] = value
    return result
